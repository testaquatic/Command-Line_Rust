use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::{Arg, ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `uniq`의 러스트 버전
pub struct Args {
    /// 입력 파일
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,
    /// 출력 파일
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,
    /// 횟수를 보여준다.
    #[arg(short, long)]
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

    /// `Args`를 실행한다.
    pub fn run(&self) -> Result<(), anyhow::Error> {
        // 읽기 파일 객체
        let mut file =
            open(self.in_file.as_str()).map_err(|e| anyhow::anyhow!("{}: {e}", self.in_file))?;
        // 현재 라인
        let mut line = String::new();
        // 이전 라인
        let mut prev_line = Option::<String>::None;
        // 같은 라인의 수
        let mut count = 1_usize;
        // 쓰기 파일 객체
        let mut writer = creae_file(self.out_file.as_deref())?;

        // 반드시 prev_line은 Some이어야 한다.
        let mut print_line = |count: usize, line: &str| {
            let line = line.trim_end_matches('\n');
            // --count
            if self.count {
                writeln!(&mut writer, "{count:>7} {line}")
            } else {
                writeln!(&mut writer, "{line}")
            }
        };

        loop {
            let bytes = file.read_line(&mut line)?;
            // EOF를 만나면 루프를 종료한다.
            if bytes == 0 {
                break;
            }

            // prev_line이 저정되어 있어야 진입한다.
            if let Some(prev_line_inner) = &prev_line {
                // 이전 라인과 같을 때
                // 마지막 '\n' 문자를 제거한 이후에 비교한다.
                // `trim()`과 `trim_end_matches('\n')`중에 하나를 골라야 하는데 애매하다.
                if prev_line_inner
                    .trim_end_matches('\n')
                    .eq(line.trim_end_matches('\n'))
                {
                    count += 1;
                } else {
                    print_line(count, prev_line_inner)?;
                    (prev_line, line) = (Some(line), prev_line.unwrap_or_default());
                    count = 1;
                }
            } else {
                // 버퍼를 재사용한다.
                (prev_line, line) = (Some(line), prev_line.unwrap_or_default());
            }

            // String버퍼를 비운다.
            line.clear();
        }

        // 출력하지 않은 마지막 줄을 처리한다.
        if let Some(inner_prev_line) = prev_line {
            print_line(count, &inner_prev_line)?;
        }

        // 러스트에서 별 의미는 없지만 버퍼를 비운다.
        writer.flush()?;

        Ok(())
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/// 쓰기 위한 파일 객체를 생성한다.
fn creae_file(filename: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    match filename {
        Some(filename) => Ok(Box::new(BufWriter::new(File::create(filename)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout().lock()))),
    }
}
