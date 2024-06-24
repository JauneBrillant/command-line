use assert_cmd::Command;

// Command::cargo_bin()
// cargoプロジェクトのバイナリを実行する
// target/debug/{ hello, true, false }を実行してくれる

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_command_exit_code_is_0() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_command_exit_code_is_1() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
