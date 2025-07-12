use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;
use crates_io_api::SyncClient;
use indicatif::{ProgressBar, ProgressStyle};
use semver::{Prerelease, Version, VersionReq};
use std::collections::{HashMap, HashSet};
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser, Debug)]
struct Args {
    /// Whether to include pre-release versions in the check.
    #[arg(long, default_value = "false")]
    prerelease: bool,
    /// Whether to hide the progress bar.
    #[arg(long, default_value = "false")]
    hidden: bool,
}

fn get_latest(client: &SyncClient, package_name: &str, prerelease: bool) -> Option<Version> {
    // Get package infomation.
    let crate_response = client.get_crate(package_name).unwrap();

    // Skip versions we can't parse.
    let versions = crate_response
        .versions
        .iter()
        .filter_map(|v| Version::parse(&v.num).ok())
        .collect::<Vec<_>>();

    // Get the latest version that isn't a pre-release.
    // If it cannot get a non pre-release version, get the latest pre-release.
    // If it cannot get a release, print a message and return.
    let latest_release = versions.iter().filter(|v| v.pre == Prerelease::EMPTY).max();
    let latest_prerelease = prerelease.then(|| versions.iter().max()).flatten();
    latest_release.or(latest_prerelease).cloned()
}

// TODO Add ability to specify strictness (e.g. latest major, or latest major-minor or latest major-minor-patch etc.)
fn main() -> ExitCode {
    let Args { prerelease, hidden } = Args::parse();

    // Get metadata for current crate/workspace
    let metadata = MetadataCommand::new()
        .current_dir(std::env::current_dir().unwrap())
        .features(CargoOpt::AllFeatures)
        .exec()
        .expect("Failed to read cargo metadata");

    let client = SyncClient::new(
        "cargo-maintained (github.com/JonathanWoollett-Light/cargo-maintained)",
        Duration::from_millis(100),
    )
    .expect("Failed to create crates.io client");

    let style = ProgressStyle::with_template(
        "{bar} {pos}/{len} [{elapsed_precise} / {eta_precise}] {per_sec}",
    )
    .unwrap();
    let pb = if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(metadata.packages.len() as u64)
    }
    .with_style(style);

    // Dependency -> (Latest version, [Crates pulling it in and their requirements])
    #[allow(clippy::type_complexity)]
    let mut dependencies: HashMap<String, Option<(Version, HashMap<String, VersionReq>)>> =
        HashMap::new();
    // Crate -> Latest version
    let mut latest_cache = HashMap::new();

    // Iterate over all the packages used.
    let number_of_packages = metadata.packages.len();
    for package in metadata.packages.into_iter() {
        let package_name = package.name.to_string();
        let is_crates_io = package.source.is_some_and(|s| s.is_crates_io());
        if !is_crates_io {
            continue;
        }
        let package_latest = latest_cache
            .entry(package_name.clone())
            .or_insert_with(|| get_latest(&client, &package_name, prerelease));
        let Some(package_latest) = package_latest else {
            continue;
        };

        let deps = client
            .crate_dependencies(&package_name, &package_latest.to_string())
            .unwrap();

        for dep in deps {
            let req = VersionReq::parse(&dep.req).unwrap();

            dependencies
                .entry(dep.crate_id.clone())
                .and_modify(|opt| {
                    if let Some((_, pulls)) = opt {
                        pulls.insert(package_name.to_string(), req.clone());
                    }
                })
                .or_insert_with(|| {
                    let dependency_latest = latest_cache
                        .entry(dep.crate_id.clone())
                        .or_insert_with(|| get_latest(&client, &dep.crate_id, prerelease));
                    dependency_latest
                        .clone()
                        .map(|dl| (dl.clone(), HashMap::from([(package_name.clone(), req)])))
                });
        }
        pb.inc(1);
    }
    pb.finish();

    let mut all_latest = true;
    let mut villains = HashSet::new();
    for (_dep, (latest, pulls)) in dependencies
        .into_iter()
        .filter_map(|(x, y)| y.map(|z| (x, z)))
    {
        for (pull, req) in pulls {
            let allows_latest = req.matches(&latest);
            all_latest &= allows_latest;
            if !allows_latest {
                villains.insert(pull);
            }
        }
    }

    if all_latest {
        println!("All of the {number_of_packages} dependencies are up to date.",);
        ExitCode::SUCCESS
    } else {
        println!("Some of the {number_of_packages} dependencies are not up to date.",);
        println!("There are {} offending crates.", villains.len());
        println!("The offending crates are {villains:?}");
        ExitCode::FAILURE
    }
}
