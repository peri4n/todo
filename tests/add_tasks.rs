use assert_cmd::Command;

#[test]
fn add_tasks() {
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("init").assert().success();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("add")
        .arg("--name")
        .arg("task")
        .arg("--due")
        .arg("2021-12-21T12:00:00")
        .assert()
        .success();

    cmd.assert().success();
}
