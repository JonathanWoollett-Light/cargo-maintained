use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

const TEMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");
const BIN: &str = env!("CARGO_BIN_EXE_cargo-maintained");
const MAN_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn new() {
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

    // println!("stdout: {}", String::from_utf8_lossy(&_cmd.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&_cmd.stderr));

    let _exe = Command::new(BIN)
        .current_dir(&crate_dir)
        // .stdout(Stdio::inherit())
        .output()
        .unwrap();
    println!("Executed checker");

    // println!("stdout: {}", String::from_utf8_lossy(&_exe.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&_exe.stderr));
}

#[test]
fn this() {
    let _exe = Command::new(BIN)
        .current_dir(&MAN_DIR)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    // println!("Executed checker");

    // println!("stdout: {}", String::from_utf8_lossy(&_exe.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&_exe.stderr));
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

    let _exe = Command::new(BIN)
        .current_dir(&crate_dir)
        // .stdout(Stdio::inherit())
        // .stderr(Stdio::inherit())
        .output()
        .unwrap();
    println!("Executed checker");
}
