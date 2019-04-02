use std::path::PathBuf;
use std::process::{Command, Output};

// ---------------------
// Test includes (start)
// ---------------------

macro_rules! win_input_path {
    () => {
        "..\\..\\resources\\rainbow_8x6.bmp"
    };
}

macro_rules! nix_input_path {
    () => {
        "resources/rainbow_8x6.bmp"
    };
}

macro_rules! input_path {
    () => {
        if cfg!(windows) {
            win_input_path!()
        } else {
            nix_input_path!()
        }
    };
}

// probably should refactor to use absolute paths instead
#[cfg(windows)]
const fn cat<'a>() -> &'a str {
    concat!("type ", win_input_path!(), " | ")
}

#[cfg(unix)]
const fn cat<'a>() -> &'a str {
    concat!("cat ", nix_input_path!(), " | ")
}

fn run_command(command: &str) -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .output()
            .expect("failed to execute process")
    }
}

// ---------------------
// Test includes (end)
// ---------------------

#[test]
fn test_input_exists() {
    let path = input_path!();
    assert!(std::path::Path::new(path).exists());
}

#[test]
fn stdin_convert_outfile() {
    let temp_dir = tempfile::tempdir().unwrap();
    let out = temp_dir.path().join("out.png");
    let out_str = out.as_path().to_str().unwrap();
    let command = format!("{}cargo run --bin convert -- -o {} ", cat(), out_str);

    println!("cmd: {:?}", command);

    let done = run_command(&command);
    assert!(done.status.success());

    let original = image::open(input_path!()).unwrap();
    let transformed = image::open(out).unwrap();
    assert_eq!(original.raw_pixels(), transformed.raw_pixels());
}

#[test]
fn stdin_blur_outfile() {
    let temp_dir = tempfile::tempdir().unwrap();
    let out = temp_dir.path().join("out.png");
    let out_str = out.as_path().to_str().unwrap();
    let command = format!("{}cargo run --bin blur -- -o {} 1", cat(), out_str);

    println!("cmd: {:?}", command);

    let done = run_command(&command);
    assert!(done.status.success());

    let original = image::open(input_path!()).unwrap();
    let transformed = image::open(out).unwrap();
    assert_ne!(original.raw_pixels(), transformed.raw_pixels());
}
