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
                let f = match open(filename) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("headr: {}: {}", filename, e);
                        return None;
                    }
                };

                if filename_print {
                    if f_num != 0 {
                        println!();
                    }
                    println!("==> {filename} <==");
                }

                Some(f)
            })
            .map(|mut f| match self.bytes {
                Some(c) => {
                    let mut bytes = vec![0; c as usize];
                    let mut bytes_stop = 0;

                    // 책의 코드로 변경함
                    loop {
                        match f.read(&mut bytes[bytes_stop..]) {
                            // EOF이거나 버퍼의 길이가 0일때
                            Ok(0) => break,
                            // n바이트만큼 읽음
                            // `Read::read`는 버퍼를 모두 채우지 못할 수가 있음
                            Ok(n) => {
                                bytes_stop += n;
                            }
                            // 재시도할 수 있는 오류
                            Err(e) if e.kind() == io::ErrorKind::Interrupted => (),
                            // 오류가 발생하면 읽기 중지
                            Err(e) => return Err(e),
                        }
                    }

                    let s = String::from_utf8_lossy(&bytes[..bytes_stop]);
                    print!("{s}");

                    Ok(())
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
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    }
                }
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
