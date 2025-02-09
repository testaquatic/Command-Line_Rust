use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use pretty_assertions::assert_eq;

#[test]
fn works() {
    assert!(true);
}

#[test]
#[should_panic]
fn not_works() {
    assert!(false);
}

#[test]
fn runs() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
#[should_panic]
fn hello_run() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn hello_run2() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
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
