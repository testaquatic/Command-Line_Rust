use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{value_parser, Parser};

#[derive(Debug, Parser)]
#[command(name = "headr", version, author, about)]
pub struct Args {
    /// 입력 파일
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// 줄 수
    #[arg(short('n'), long, default_value = "10", value_name = "LINES", value_parser = value_parser!(u64).range(1..))]
    lines: u64,

    /// 바이트수
    #[arg(short('c'), long, value_name = "BYTES", conflicts_with("lines"), value_parser = value_parser!(u64).range(1.. ))]
    bytes: Option<u64>,
}

impl Args {
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
