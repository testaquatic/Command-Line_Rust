use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, Command};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("TestAquatic")
        .about("러스트로 작성한 `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .conflicts_with("number_nonblank_lines")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        // `.num_args()`를 `1..`으로 설정했으므로 `unwrap`을 사용해도 안전하다.
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

fn run(args: Args) -> Result<(), anyhow::Error> {
    let mut line_num: std::ops::RangeFrom<usize> = 1_usize..;
    args.files.iter().try_for_each(|filename| {
        let b_reader = match open(filename) {
            Err(e) => {
                eprintln!("catr: {}: {}", filename, e);
                return Ok(());
            }
            Ok(b_reader) => b_reader,
        };

        let line_reader = b_reader.lines();

        if args.number_lines {
            line_reader
                .into_iter()
                .zip(line_num.by_ref())
                .try_for_each(|(line, n)| {
                    println!("{:>6}\t{}", n, line?);

                    Result::<(), io::Error>::Ok(())
                })?;
        } else if args.number_nonblank_lines {
            line_reader.into_iter().try_for_each(|line| {
                let line = line?;
                if line.is_empty() {
                    println!();
                    return Ok(());
                }
                // "1_usize.."는 None을 반환할 가능성이 거의 없다.
                println!("{:>6}\t{}", line_num.next().unwrap().to_string(), line);

                Result::<(), io::Error>::Ok(())
            })?;
        } else {
            line_reader.into_iter().try_for_each(|line| {
                println!("{}", line?);

                Result::<(), io::Error>::Ok(())
            })?
        }

        Result::<(), anyhow::Error>::Ok(())
    })?;

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
