use assert_cmd::Command;

#[test]
fn start_the_app() {
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.assert().success();
}

#[test]
fn print_the_version() {
    let mut cmd = Command::cargo_bin("todo").unwrap();

    let assert = cmd.arg("--version")
        .assert();

    assert.success()
        .stdout("todo 0.1.0\n");
}
