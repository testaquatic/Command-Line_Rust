use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{ArgAction, Command};

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    /// 결과를 누적한다.
    fn add(&mut self, rhs: &Self) {
        self.num_lines += rhs.num_lines;
        self.num_words += rhs.num_words;
        self.num_bytes += rhs.num_bytes;
        self.num_chars += rhs.num_chars;
    }
}

/// `FileInfo`의 내용을 stdout에 출력한다.
#[derive(Debug)]
struct FileInfoPrint<'a> {
    /// 파일이 여러개 일 때 사용하기 위한 벡터
    fileinfos: Vec<Vec<String>>,
    /// 문자열의 최대길이
    /// GNU버전은 지정이 필요하다.
    max: usize,
    /// 출력할 항목을 선택하기 위한 참조자
    args: &'a Args,
}

impl<'a> FileInfoPrint<'a> {
    /// `FileInfoPrint`를 생성한다.
    fn new(args: &'a Args) -> FileInfoPrint<'a> {
        FileInfoPrint {
            fileinfos: Vec::new(),
            max: if args.files.len() > 1 { 3 } else { 0 },
            args,
        }
    }

    /// 출력한 `FileInfo`를 추가한다.
    fn append(
        &mut self,
        filename: &str,
        &FileInfo {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }: &FileInfo,
    ) {
        let mut line = Vec::new();

        // 항목 추가를 쉽게 하기 위한 클로저
        let mut line_item_add = |check: bool, count: usize| {
            check.then(|| line.push(count.to_string()));
        };
        line_item_add(self.args.lines, num_lines);
        line_item_add(self.args.words, num_words);
        line_item_add(self.args.bytes, num_bytes);
        line_item_add(self.args.chars, num_chars);

        line.iter().for_each(|count| {
            // GNU버전은 마지막이 9로 끝나면 공백이 추가된다.
            // 더 명료한 방법이 있을 것 같지만 일단 이 방법으로 처리한다.
            if count.ends_with("9") && line.len() > 1 {
                self.max = self.max.max(count.len() + 1);
            } else {
                self.max = self.max.max(count.len());
            }
        });

        line.push(filename.to_string());

        self.fileinfos.push(line);
    }

    /// 결과를 GNU버전에 맞춰서 출력한다.
    fn print(&self) {
        self.fileinfos.iter().for_each(|fileinfo| {
            fileinfo
                .iter()
                .enumerate()
                // 벡터의 길이는 최소한 2이다.
                // 마지막 경로명은 따로 처리한다.
                .take(fileinfo.len() - 1)
                .for_each(|(idx, count)| {
                    print!(
                        "{:>width$}{}",
                        count,
                        // 마지막에 공백을 추가한다.
                        // 경로명일 때는 공백을 추가하지 않는다.
                        if idx == fileinfo.len() - 2 { "" } else { " " },
                        width = self.max
                    );
                });
            // 벡터의 길이가 0이 될 수 없다.
            // `unwrap`을 사용할 수 있다.
            let filename = fileinfo.last().unwrap();
            // 입력이 stdin일 때는 경로명을 출력하지 않는다.
            if filename == "-" {
                println!();
            } else {
                println!(" {}", filename);
            }
        });
    }
}

#[derive(Debug)]
pub struct Args {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("wcr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("Rust version of `wc`")
            .arg(
                clap::Arg::new("files")
                    .value_name("FILE")
                    .num_args(0..)
                    .default_value("-")
                    .help("Input file(s)"),
            )
            .arg(
                clap::Arg::new("lines")
                    .short('l')
                    .long("lines")
                    .help("Show line count")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                clap::Arg::new("words")
                    .short('w')
                    .long("words")
                    .help("Show word count")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                clap::Arg::new("bytes")
                    .short('c')
                    .long("bytes")
                    .help("Show byte count")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                clap::Arg::new("chars")
                    .short('m')
                    .long("chars")
                    .help("Show character count")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("bytes"),
            )
            .get_matches();

        Self {
            files: matches.get_many("files").unwrap().cloned().collect(),
            lines: matches.get_flag("lines"),
            words: matches.get_flag("words"),
            chars: matches.get_flag("chars"),
            bytes: matches.get_flag("bytes"),
        }
    }

    /// 인수를 조정한다.
    fn args_adjustment(&mut self) {
        if !(self.lines || self.words || self.bytes || self.chars) {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        self.args_adjustment();

        // 전체 집계를 담당하는 객체이다.
        let mut total = FileInfo {
            num_bytes: 0,
            num_chars: 0,
            num_lines: 0,
            num_words: 0,
        };
        // 출력을 위한 객체이다.
        let mut printfileinfos = FileInfoPrint::new(self);
        self.files.iter().try_for_each(|filename| {
            match open(filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(_) => {
                    let count = count(open(filename)?)?;
                    // 출력할 항목을 추가한다.
                    printfileinfos.append(filename, &count);
                    // 파일의 수가 1보다 많을 때는 `total`을 갱신해야 한다.
                    self.files.len().gt(&1).then(|| total.add(&count));
                }
            }

            Result::<(), io::Error>::Ok(())
        })?;

        // `total`을 출력하도록 추가한다.
        self.files.len().gt(&1).then(|| {
            printfileinfos.append("total", &total);
        });

        // 결과를 출력한다.
        printfileinfos.print();

        Ok(())
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo, io::Error> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        // EOF
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += line_bytes;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        pretty_assertions::assert_eq!(info.unwrap(), expected);
    }
}
