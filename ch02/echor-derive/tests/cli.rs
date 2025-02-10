use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

#[test]
fn dies_no_args() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<(), anyhow::Error> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?.args(args).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    pretty_assertions::assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn hello1() -> Result<(), anyhow::Error> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<(), anyhow::Error> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<(), anyhow::Error> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<(), anyhow::Error> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
