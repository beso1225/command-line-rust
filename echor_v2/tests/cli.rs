use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // let mut cmd = Command::cargo_bin("echor").unwrap();
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("Usage"));
    Command::cargo_bin("echor_v2")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// #[test]
// fn runs() {
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.arg("hello")
//         .assert()
//         .success();
// }

// #[test]
// fn hello1() -> TestResult {
//     let outfile = "tests/expected/hello1.txt";
//     let expected = fs::read_to_string(outfile)?;
//     let mut cmd = Command::cargo_bin("echor")?;
//     cmd.arg("Hello there").assert().success().stdout(expected);
//     Ok(())
// }

// #[test]
// fn hello2() -> TestResult {
//     let expected = fs::read_to_string("tests/expected/hello2.txt")?;
//     let mut cmd = Command::cargo_bin("echor")?;
//     cmd.args(vec!["Hello", "there"])
//         .assert()
//         .success()
//         .stdout(expected);
//     Ok(())
// }

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor_v2")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}