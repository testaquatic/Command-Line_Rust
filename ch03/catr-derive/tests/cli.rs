use std::fs;

use assert_cmd::cargo::CargoError;
use assert_cmd::Command;
use rand::distributions::Alphanumeric;
use rand::Rng;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> Result<(), anyhow::Error> {
    ["-h", "--help"]
        .iter()
        .try_for_each(|flag| -> Result<(), CargoError> {
            Command::cargo_bin(PRG)?
                .arg(flag)
                .assert()
                .stdout(predicates::str::contains("Usage"));
            Ok(())
        })?;

    Ok(())
}

fn get_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> Result<(), anyhow::Error> {
    let bad = get_bad_file();
    let expected = format!(r#"{PRG}: {bad}: .* \(os error 2\)"#);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<(), anyhow::Error> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    pretty_assertions::assert_eq!(stdout, expected);

    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> Result<(), anyhow::Error> {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    pretty_assertions::assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn bustle_stdin() -> Result<(), anyhow::Error> {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

#[test]
fn empty() -> Result<(), anyhow::Error> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}
// --------------------------------------------------
#[test]
fn empty_b() -> Result<(), anyhow::Error> {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> Result<(), anyhow::Error> {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> Result<(), anyhow::Error> {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> Result<(), anyhow::Error> {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> Result<(), anyhow::Error> {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> Result<(), anyhow::Error> {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> Result<(), anyhow::Error> {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> Result<(), anyhow::Error> {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> Result<(), anyhow::Error> {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> Result<(), anyhow::Error> {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// --------------------------------------------------
#[test]
fn all() -> Result<(), anyhow::Error> {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> Result<(), anyhow::Error> {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> Result<(), anyhow::Error> {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
