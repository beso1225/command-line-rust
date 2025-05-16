// #[test]
// fn works() {
//     // assert!(true);
//     assert!(false);
// }

// use std::process::Command;

// #[test]
// fn runs() {
//     // let mut cmd = Command::new("ls");
//     let mut cmd = Command::new("hello");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }

use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}