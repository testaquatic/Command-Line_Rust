use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{value_parser, Arg, Command};

#[derive(Debug)]
pub struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("headr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("러스트로 만든 `head`")
            .arg(
                Arg::new("files")
                    .help("Input file(s)")
                    .value_name("FILE")
                    .default_value("-")
                    .num_args(0..),
            )
            .arg(
                Arg::new("lines")
                    .help("Number of lines")
                    .default_value("10")
                    .short('n')
                    .long("lines")
                    .value_name("LINES")
                    .value_parser(value_parser!(u64).range(1..))
                    .conflicts_with("bytes"),
            )
            .arg(
                Arg::new("bytes")
                    .help("Number of bytes")
                    .short('c')
                    .long("bytes")
                    .value_name("BYTES")
                    .value_parser(value_parser!(u64).range(1..))
                    .conflicts_with("lines"),
            )
            .get_matches();

        Args {
            // 인수의 개수를 1..으로 지정했으므로 unwrap을 사용해도 안전하다.
            files: matches.get_many("files").unwrap().cloned().collect(),
            // 기본값이 지정되어 있으므로 unwrap을 사용해도 안전하다.
            lines: matches.get_one("lines").cloned().unwrap(),
            bytes: matches.get_one("bytes").cloned(),
        }
    }

    pub fn run(self) -> Result<(), anyhow::Error> {
        let mut line_string = String::new();
        let f_len = self.files.len();
        let filename_print = f_len > 1;

        self.files
            .iter()
            .enumerate()
            .filter_map(|(f_num, filename)| {
                let mut f = match open(filename) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("{}: {}", filename, e);
                        return None;
                    }
                };

                if filename_print {
                    println!("==> {filename} <==");
                }

                let result = match self.bytes {
                    Some(c) => {
                        let bytes = f
                            .bytes()
                            .take(c as usize)
                            .collect::<Result<Vec<_>, io::Error>>();
                        let bytes = match bytes {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                return Some(Err(e));
                            }
                        };
                        let s = String::from_utf8_lossy(&bytes);
                        print!("{s}");

                        Some(Ok(()))
                    }
                    None => {
                        let line_result = (0..self.lines)
                            .map_while(|_| match f.read_line(&mut line_string) {
                                Ok(0) => None,
                                Ok(n) => {
                                    print!("{line_string}");
                                    line_string.clear();
                                    Some(Ok(n))
                                }
                                Err(e) => Some(Err(e)),
                            })
                            .try_for_each(|line_result| match line_result {
                                Ok(_) => Ok(()),
                                Err(e) => Err(e),
                            });

                        match line_result {
                            Ok(_) => Some(Ok(())),
                            Err(e) => Some(Err(e)),
                        }
                    }
                };

                if filename_print && f_num < f_len - 1 {
                    println!();
                }

                result
            })
            .try_for_each(|result| result)?;

        Ok(())
    }
}

pub fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
