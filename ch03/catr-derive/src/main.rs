use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `cat`의 러스트 버전
struct Args {
    /// 입력 파일
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// 줄 번호를 매긴다.
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// 빈 줄이 아닐 때만 번호를 매긴다.
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
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
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
