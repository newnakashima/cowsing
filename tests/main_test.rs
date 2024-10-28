use std::process::Command;

#[test]
fn test_main() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello world");
}
