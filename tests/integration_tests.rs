use std::process::Command;

#[test]
fn prints_help_log() {
    let output = Command::new("target/debug/dashlight")
        .arg("-h")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let first_line = stderr.lines().next().unwrap();

    assert!(first_line.contains("Usage: dashlight"));
    assert!(stdout.is_empty());
}

// Extremely naive test to check that it's (probably) adding up the log correctly
// When I have a stable machine-output format I'll parse that properly
#[test]
fn watch_test() {
    let output = Command::new("target/debug/dashlight")
        .arg("watch")
        .arg("-f")
        .arg("tests/data/short-log")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("     5 |     2 |     2 |     1 |     0 "));
    assert!(stdout.contains("         / |     1 |     2 |     0 |     0 "));
}

#[test]
fn convert_test() {
    let output = Command::new("target/debug/dashlight")
        .arg("convert")
        .arg("-f")
        .arg("tests/data/short-log")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#"43.183.122.65"09/May/2022:00:00:07 +0000"GET"/"200"#));
    assert!(stdout.contains(r#"43.193.122.65"09/May/2022:00:00:07 +0000"POST"/api/user"403"#));
}

#[test]
fn print_help() {
    let output = Command::new("target/debug/dashlight")
        .arg("-h")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let first_line = stderr.lines().next().unwrap();

    assert!(first_line.contains("Usage: dashlight"));
    assert!(stdout.is_empty());
}
