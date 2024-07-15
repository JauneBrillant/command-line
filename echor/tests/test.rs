use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut echor = Command::cargo_bin("echor").unwrap();
    echor.arg("hello world !").assert().success();
}

#[test]
fn dies_no_args() {
    let mut echor = Command::cargo_bin("echor").unwrap();
    echor
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

// #[test]
// fn test1() {
//     let args = "hello world !\n";
//     let mut echor = Command::cargo_bin("echor").unwrap();
//     echor.arg(args);
//     echor.assert().success().stdout("hello world !\n");
// }

// #[test]
// fn test2() {
//     let args = "a b c";
//     let mut echor = Command::cargo_bin("echor").unwrap();
//     echor.arg(args);
//     echor.assert().success().stdout(args);
// }
