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
    fn add(&mut self, rhs: &Self) {
        self.num_lines += rhs.num_lines;
        self.num_words += rhs.num_words;
        self.num_bytes += rhs.num_bytes;
        self.num_chars += rhs.num_chars;
    }
}

#[derive(Debug)]
struct PrintFileInfos<'a> {
    fileinfos: Vec<Vec<String>>,
    max: usize,
    args: &'a Args,
}

impl<'a> PrintFileInfos<'a> {
    fn new(args: &'a Args) -> PrintFileInfos<'a> {
        PrintFileInfos {
            fileinfos: Vec::new(),
            max: if args.files.len() > 1 { 3 } else { 0 },
            args,
        }
    }

    fn append(
        &mut self,
        filename: &str,
        FileInfo {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }: &FileInfo,
    ) {
        let mut line = Vec::new();

        let mut line_item_add = |count: usize| {
            line.push(count.to_string());
        };

        self.args.lines.then(|| line_item_add(*num_lines));
        self.args.words.then(|| line_item_add(*num_words));
        self.args.bytes.then(|| line_item_add(*num_bytes));
        self.args.chars.then(|| line_item_add(*num_chars));

        line.iter().for_each(|count| {
            if count.ends_with("9") && line.len() > 1 {
                self.max = self.max.max(count.len() + 1);
            } else {
                self.max = self.max.max(count.len());
            }
        });

        line.push(filename.to_string());

        self.fileinfos.push(line);
    }

    fn print(&self) {
        // dbg!(&self.args);
        self.fileinfos.iter().for_each(|fileinfo| {
            fileinfo
                .iter()
                .enumerate()
                // 벡터의 길이는 최소한 2이다.
                .take(fileinfo.len() - 1)
                .for_each(|(idx, count)| {
                    print!(
                        "{:>width$}{}",
                        count,
                        if idx == fileinfo.len() - 2 { "" } else { " " },
                        width = self.max
                    );
                });
            // 벡터의 길이가 0이 될 수 없다.
            let filename = fileinfo.last().unwrap();
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

        let mut lines = matches.get_flag("lines");
        let mut words = matches.get_flag("words");
        let mut bytes = matches.get_flag("bytes");
        let chars = matches.get_flag("chars");
        if !(lines || words || bytes || chars) {
            lines = true;
            words = true;
            bytes = true;
        }

        Self {
            files: matches.get_many("files").unwrap().cloned().collect(),
            lines,
            words,
            chars,
            bytes,
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        let mut total = FileInfo {
            num_bytes: 0,
            num_chars: 0,
            num_lines: 0,
            num_words: 0,
        };
        let mut printfileinfos = PrintFileInfos::new(self);
        self.files.iter().try_for_each(|filename| {
            match open(filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(_) => {
                    let count = count(open(filename)?)?;
                    printfileinfos.append(filename, &count);
                    (self.files.len() > 1).then(|| total.add(&count));
                }
            }

            Result::<(), io::Error>::Ok(())
        })?;

        self.files.len().gt(&1).then(|| {
            printfileinfos.append("total", &total);
        });

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
