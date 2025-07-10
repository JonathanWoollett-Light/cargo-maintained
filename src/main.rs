use ansi_term::Colour::{Green, Red};
use cargo_metadata::{CargoOpt, MetadataCommand};
use crates_io_api::{CrateResponse, Dependency, SyncClient};
use semver::{Op, Prerelease, Version, VersionReq};
use std::collections::{HashMap, HashSet};
use std::process::ExitCode;
use std::time::Duration;

const INDENT: &str = "  ";

// TODO Filter by active features.
// TODO Add a progress bar, we can make rough estimates based on how long it
// takes to cover each item, if we have 6 items at depth 0, we split progress
// bar in /6 if we have 4 at the 1st we can assume each of the 6 segments will
// take 4*6*time per item.
// TODO Add ability to specify strictness (e.g. latest major, or latest major-minor or latest major-minor-patch etc.)
fn main() -> ExitCode {
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

    let mut crate_cache = HashMap::new();
    let mut deps_cache = HashMap::new();
    let mut count = 0;
    let max_depth = 1;
    let mut villains = HashSet::new();

    let latest = metadata.packages.iter().fold(true, |acc, pkg| {
        let latest_pkg_deps = pkg.dependencies.iter().fold(true, |acc, dep| {
            // Skip non-crates.io dependencies
            let latest_deps = match &dep.source {
                Some(source) if source.starts_with("registry+") => handle_pkg(
                    &dep.name,
                    &dep.req,
                    1,
                    &client,
                    &mut crate_cache,
                    &mut deps_cache,
                    &mut count,
                    max_depth,
                    &pkg.name,
                    &mut villains,
                ),
                _ => {
                    println!("skipping {}", dep.name);
                    true
                } // Ignore non-crates.io dependencies
            };

            acc && latest_deps
        });
        acc && latest_pkg_deps
    });

    if latest {
        println!("All of the {count} dependencies are up to date.");
        ExitCode::SUCCESS
    } else {
        println!("Some of the {count} dependencies are not up to date.");
        println!("There are {} offending crates", villains.len());
        println!("The offending crates are {villains:?}");
        ExitCode::FAILURE
    }
}

fn handle_pkg(
    name: &str,
    version: &VersionReq,
    depth: usize,
    client: &SyncClient,
    // Stores cached crates.io info about crates.
    crate_cache: &mut HashMap<String, CrateResponse>,
    // Stores cached dependencies of the latest versions of crates.
    deps_cache: &mut HashMap<String, Vec<Dependency>>,
    // The total number of crates checked.
    count: &mut usize,
    // The maximum depth to search in the dependency tree,
    max_depth: usize,
    parent: &str,
    // The crates not using latest major versions.
    villains: &mut HashSet<String>,
) -> bool {
    let prefix = format!("{}{name}", INDENT.repeat(depth+1));
    // println!("got here: {}",name);

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
    let latest_prerelease = versions.iter().max();
    let Some(latest) = latest_release.or(latest_prerelease) else {
        println!("{prefix} can't find release");
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

    if depth >= max_depth {
        println!(" reached max depth");
        return allows_latest_major;
    }

    // If the crate was cached we have already checked its dependencies.
    if deps_cache.contains_key(name) {
        println!(" dependencies already checked");
        return allows_latest_major;
    }

    println!();
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
            crate_cache,
            deps_cache,
            count,
            max_depth,
            name,
            villains,
        );
        acc && latest_pkg
    });

    allows_latest_major && deps_latest
}
