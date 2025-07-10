use ansi_term::Colour::{Green, Red};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;
use crates_io_api::{CrateResponse, Dependency, SyncClient};
use semver::{Op, Prerelease, Version, VersionReq};
use std::collections::{HashMap, HashSet};
use std::process::ExitCode;
use std::time::Duration;

const INDENT: &str = "  ";

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "1")]
    max_depth: usize,
    #[arg(long, default_value = "false")]
    prerelease: bool,
    #[arg(long, default_value = "false")]
    tree: bool,
}

#[derive(Debug)]
struct ImmutableState {
    max_depth: usize,
    prerelease: bool,
    tree: bool,
}

#[derive(Debug, Default)]
struct MutableState {
    /// Stores cached crates.io info about crates.
    crate_cache: HashMap<String, CrateResponse>,
    /// Stores cached dependencies of the latest versions of crates.
    deps_cache: HashMap<String, Vec<Dependency>>,
    /// The total number of crates checked.
    count: usize,
    /// The crates not using latest major versions.
    villains: HashSet<String>,
}

// TODO Filter by active features.
// TODO Add a progress bar, we can make rough estimates based on how long it
// takes to cover each item, if we have 6 items at depth 0, we split progress
// bar in /6 if we have 4 at the 1st we can assume each of the 6 segments will
// take 4*6*time per item.
// TODO Add ability to specify strictness (e.g. latest major, or latest major-minor or latest major-minor-patch etc.)
fn main() -> ExitCode {
    let args = Args::parse();

    // Get metadata for current crate/workspace
    let metadata = MetadataCommand::new()
        .current_dir(std::env::current_dir().unwrap())
        .features(CargoOpt::AllFeatures)
        .exec()
        .expect("Failed to read cargo metadata");

    let client = SyncClient::new(
        "dependency-checker (github.com/yourname/dependency-checker)",
        Duration::from_millis(100),
    )
    .expect("Failed to create crates.io client");

    let mut mutable_state = MutableState::default();
    let immutable_state = ImmutableState {
        max_depth: args.max_depth,
        prerelease: args.prerelease,
        tree: args.tree,
    };

    let latest = metadata.packages.iter().fold(true, |acc, pkg| {
        let latest_pkg_deps = pkg.dependencies.iter().fold(true, |acc, dep| {
            // Skip non-crates.io dependencies
            let latest_deps = match &dep.source {
                Some(source) if source.starts_with("registry+") => handle_pkg(
                    &dep.name,
                    &dep.req,
                    1,
                    &client,
                    &pkg.name,
                    &immutable_state,
                    &mut mutable_state,
                ),
                _ => {
                    if immutable_state.tree {
                        println!("skipping {}", dep.name);
                    }
                    true
                } // Ignore non-crates.io dependencies
            };

            acc && latest_deps
        });
        acc && latest_pkg_deps
    });

    if latest {
        println!(
            "All of the {} dependencies are up to date.",
            mutable_state.count
        );
        ExitCode::SUCCESS
    } else {
        println!(
            "Some of the {} dependencies are not up to date.",
            mutable_state.count
        );
        println!(
            "There are {} offending crates",
            mutable_state.villains.len()
        );
        println!("The offending crates are {:?}", mutable_state.villains);
        ExitCode::FAILURE
    }
}

fn handle_pkg(
    name: &str,
    version: &VersionReq,
    depth: usize,
    client: &SyncClient,
    parent: &str,
    immutable_state: &ImmutableState,
    mutable_state: &mut MutableState,
) -> bool {
    let MutableState {
        crate_cache,
        deps_cache,
        count,
        villains,
    } = mutable_state;
    let ImmutableState {
        max_depth,
        prerelease,
        tree,
    } = immutable_state;
    let prefix = format!("{}{name}", INDENT.repeat(depth + 1));

    if !crate_cache.contains_key(name) {
        *count += 1;
    }

    let entry = crate_cache
        .entry(name.to_string())
        .or_insert_with(|| client.get_crate(name).unwrap());

    // Skip versions we can't parse.
    let versions = entry
        .versions
        .iter()
        .filter_map(|v| Version::parse(&v.num).ok())
        .collect::<Vec<_>>();

    // TODO Should add optional to allow/disallow pre-release dependencies.
    // Get the latest version that isn't a pre-release.
    // If it cannot get a non pre-release version, get the latest pre-release.
    // If it cannot get a release, print a message and return.
    let latest_release = versions.iter().filter(|v| v.pre == Prerelease::EMPTY).max();
    let latest_prerelease = prerelease.then(|| versions.iter().max()).flatten();
    let Some(latest) = latest_release.or(latest_prerelease) else {
        if *tree {
            println!("{prefix} can't find release");
        }
        return true;
    };

    // TODO We can probably hightlight the specific comparator that breaks it.
    let allows_latest_major = version.comparators.iter().all(|c| {
        match c.op {
            Op::Exact => c.major == latest.major,
            Op::Greater => c.major < latest.major,
            Op::GreaterEq => c.major <= latest.major,
            Op::Less => c.major > latest.major,
            Op::LessEq => c.major >= latest.major,
            Op::Wildcard => true,                 // Wildcard allows any version
            Op::Tilde => c.major == latest.major, // Tilde allows only the same
            Op::Caret => c.major == latest.major, // Caret allows only the same major
            _ => true,                            // Allow any other operator
        }
    });

    if allows_latest_major {
        print!(
            "{}",
            Green.paint(&format!("{prefix} ({latest}) ∈ {{{version}}}"))
        );
    } else {
        print!(
            "{}",
            Red.paint(format!("{prefix} ({latest}) /∈ {{{version}}}"))
        );
        villains.insert(parent.to_string());
    };

    if depth >= *max_depth {
        if *tree {
            println!(" reached max depth");
        }
        return allows_latest_major;
    }

    // If the crate was cached we have already checked its dependencies.
    if deps_cache.contains_key(name) {
        if *tree {
            println!(" dependencies already checked");
        }
        return allows_latest_major;
    }

    if *tree {
        println!();
    }
    let deps = client
        .crate_dependencies(name, &latest.to_string())
        .unwrap();
    deps_cache.insert(name.to_string(), deps.clone());

    let deps_latest = deps.iter().fold(true, |acc, dep| {
        let latest_pkg = handle_pkg(
            &dep.crate_id,
            &VersionReq::parse(&dep.req).unwrap(),
            depth + 1,
            client,
            name,
            immutable_state,
            mutable_state,
        );
        acc && latest_pkg
    });

    allows_latest_major && deps_latest
}
