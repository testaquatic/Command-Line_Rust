use std::thread::park_timeout;

use clap::{Arg, ArgAction, Command, Parser};
use regex::RegexBuilder;

#[derive(Debug, Parser)]
#[command(version, about, author)]
/// `grep`의 러스트 버전
pub struct Args {
    /// 검색 패턴
    #[arg()]
    pattern: String,
    /// 파일(들)
    #[arg(value_name = "FILE", default_value = "-", num_args(1..))]
    files: Vec<String>,
    /// 대소문자 구분 없이 검색
    #[arg(short, long, action = ArgAction::SetTrue)]
    insensitive: bool,
    /// 하위 디렉토리 검색
    #[arg(short, long, action = ArgAction::SetTrue)]
    recursive: bool,
    /// 매칭된 줄 수 출력
    #[arg(short, long, action = ArgAction::SetTrue)]
    count: bool,
    /// 매칭되지 않은 줄 수 출력
    #[arg(short = 'v', long = "invert-match", action = ArgAction::SetTrue)]
    invert: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("grepr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("`grep`의 러스트 버전")
            .arg(
                Arg::new("pattern")
                    .value_name("PATTERN")
                    .help("검색 패턴")
                    .required(true),
            )
            .arg(
                Arg::new("files")
                    .value_name("FILE")
                    .help("파일(들)")
                    .default_value("-")
                    .num_args(1..),
            )
            .arg(
                Arg::new("insensitive")
                    .help("대소문자 구분 없이 검색")
                    .short('i')
                    .long("insensitive")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("recursive")
                    .help("하위 디렉토리 검색")
                    .short('r')
                    .long("recursive")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count")
                    .help("매칭된 줄 수 출력")
                    .short('c')
                    .long("count")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("invert")
                    .help("매칭되지 않은 줄 출력")
                    .short('v')
                    .long("invert-match")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Self {
            // 반드시 필요한 인수이므로 `unwrap`를 사용할 수 있음
            pattern: matches.get_one("pattern").cloned().unwrap(),
            // 기본 인수가 지정되어 있으므로 `unwrap`을 사용할 수 있음
            files: matches.get_many("files").unwrap().cloned().collect(),
            insensitive: matches.get_flag("insensitive"),
            recursive: matches.get_flag("recursive"),
            count: matches.get_flag("count"),
            invert: matches.get_flag("invert"),
        }
    }

    fn make_regex(&self) -> Result<regex::Regex, regex::Error> {
        let mut pattern = RegexBuilder::new(&self.pattern)
            .case_insensitive(self.insensitive)
            .build()?;
        println!("pattern \"{pattern}\"");

        Ok(pattern)
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        let pattern = self.make_regex()?;

        Ok(())
    }
}
