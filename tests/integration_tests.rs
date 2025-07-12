use std::{path::PathBuf, process::Command};

const TEMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");
const BIN: &str = env!("CARGO_BIN_EXE_cargo-maintained");
const MAN_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn fast_new() {
    const NAME: &str = "new";
    let temp_dir = PathBuf::from(TEMP_DIR);
    let crate_dir = temp_dir.join(NAME);
    if crate_dir.exists() {
        std::fs::remove_dir_all(&crate_dir).unwrap();
    }

    let _cmd = Command::new("cargo")
        .arg("new")
        .arg(NAME)
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    println!("Created project");

    let exe = Command::new(BIN)
        .current_dir(&crate_dir)
        // .stdout(std::process::Stdio::inherit())
        // .stderr(std::process::Stdio::inherit())
        .arg("--hidden")
        .output()
        .unwrap();
    println!("Executed checker");

    assert!(exe.status.success());
}

#[test]
fn fast_this() {
    println!("Started checker");
    let exe = Command::new(BIN)
        .current_dir(MAN_DIR)
        // .stdout(std::process::Stdio::inherit())
        // .stderr(std::process::Stdio::inherit())
        .arg("--hidden")
        .output()
        .unwrap();
    println!("Executed checker");

    assert!(!exe.status.success());
}

#[test]
fn axum() {
    const NAME: &str = "axum";
    let temp_dir = PathBuf::from(TEMP_DIR);
    let crate_dir = temp_dir.join(NAME);
    if !crate_dir.exists() {
        let _cmd = Command::new("git")
            .arg("clone")
            .arg("https://github.com/tokio-rs/axum")
            .current_dir(&temp_dir)
            .output()
            .unwrap();
    }
    println!("Cloned axum project");

    let exe = Command::new(BIN)
        .current_dir(&crate_dir)
        // .stdout(std::process::Stdio::inherit())
        // .stderr(std::process::Stdio::inherit())
        .arg("--hidden")
        .output()
        .unwrap();
    println!("Executed checker");

    assert!(!exe.status.success());
}
