use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::{Arg, ArgAction};

#[derive(Debug)]
pub struct Args {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = clap::Command::new("uniqr")
            .about("`uniq`의 간단한 러스트 구현")
            .version("0.1.0")
            .author("TestAqutic")
            .arg(
                Arg::new("in_file")
                    .value_name("IN_FILE")
                    .help("입력 파일")
                    .default_value("-"),
            )
            .arg(
                Arg::new("out_file")
                    .value_name("OUT_FILE")
                    .help("출력 파일"),
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("count")
                    .help("횟수 표시")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Args {
            //
            in_file: matches.get_one("in_file").cloned().unwrap(),
            out_file: matches.get_one("out_file").cloned(),
            count: matches.get_flag("count"),
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        let mut file =
            open(self.in_file.as_str()).map_err(|e| anyhow::anyhow!("{}: {e}", self.in_file))?;
        let mut line = String::new();
        let mut prev_line = Option::<String>::None;
        let mut count = 1_usize;
        let mut writer = creae_file(self.out_file.as_deref())?;

        // 반드시 prev_line은 Some이어야 한다.
        let mut print_line = |count: usize, line: &str| {
            if self.count {
                writeln!(&mut writer, "{count:>7} {}", line)
            } else {
                writeln!(&mut writer, "{}", line)
            }
        };

        loop {
            let bytes = file.read_line(&mut line)?;
            // EOF를 만나면 루프를 종료한다.
            if bytes == 0 {
                break;
            }
            line = line.trim_end_matches('\n').to_string();

            if let Some(prev_line_inner) = &prev_line {
                if prev_line_inner.eq(&line) {
                    count += 1;
                } else {
                    print_line(count, prev_line_inner)?;
                    (prev_line, line) = (Some(line), prev_line.unwrap_or_default());
                    count = 1;
                }
            } else {
                (prev_line, line) = (Some(line), prev_line.unwrap_or_default());
            }

            // String버퍼를 비운다.
            line.clear();
        }
        if let Some(inner_prev_line) = prev_line {
            print_line(count, &inner_prev_line)?;
        }

        Ok(())
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn creae_file(filename: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    match filename {
        Some(filename) => Ok(Box::new(BufWriter::new(File::create(filename)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout().lock()))),
    }
}
