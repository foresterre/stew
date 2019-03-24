use std::process::{Command, Output};

fn run_command() -> Output {
    const INPUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/rainbow_8x6.bmp");
    const OUTPUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/x_blur.png");
    const VALUE: &str = "50";

    Command::new("cargo")
        .args(&[
            "run", "--bin", "blur", "--", "-i", INPUT, "-o", OUTPUT, VALUE,
        ])
        .output()
        .expect("Running test failed")
}

#[test]
fn cli_blur() {
    let res = run_command();

    assert!(res.status.success());
}
