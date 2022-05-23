use std::process::Command;

// Extremely naive test to check that it's (probably) adding up the log correctly
// When I have a stable machine-output format I'll parse that properly
#[test]
fn short_data_test() {
    let output = Command::new("target/debug/dashlight")
        .arg("data/short-log")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);
    assert!(output.contains("90"));
    assert!(output.contains("2xx: 9"));
    assert!(output.contains("3xx: 2"));
}

