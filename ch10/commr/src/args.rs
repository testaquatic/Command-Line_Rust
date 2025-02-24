mod line_iter;

use std::io;

use clap::{Arg, ArgAction, Command, Parser};
use line_iter::{LineIter, LinePosition};

use crate::open;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct Args {
    /// 입력 파일 1
    #[arg(value_name = "FILE1", required(true))]
    file1: String,
    /// 입력 파일 2
    #[arg(value_name = "FILE2", required(true))]
    file2: String,
    /// 1행을 표시하지 않음
    #[arg(short = '1', action(ArgAction::SetFalse))]
    show_col1: bool,
    /// 2행을 표시하지 않음
    #[arg(short = '2', action(ArgAction::SetFalse))]
    show_col2: bool,
    /// 3행을 표시하지 않음
    #[arg(short = '3', action(ArgAction::SetFalse))]
    show_col3: bool,
    /// 대소문자를 구분하지 않음
    #[arg(short, action(ArgAction::SetTrue))]
    insensitive: bool,
    /// 행간 구분 문자
    #[arg(
        short,
        long("output-delimiter"),
        value_name = "DELIM",
        default_value("\t")
    )]
    delimiter: String,
    /// `derive_mode`를 활성화하는 디버그용 플래그
    #[arg(long("derive_mode"), hide(true), action(ArgAction::SetTrue))]
    derive_mode: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("commr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("`comm`의 간단한 러스트 버전")
            .arg(
                Arg::new("file1")
                    .value_name("FILE1")
                    .required(true)
                    .help("입력 파일 1"),
            )
            .arg(
                Arg::new("file2")
                    .value_name("FILE2")
                    .required(true)
                    .help("입력 파일 2"),
            )
            .arg(
                Arg::new("show_col1")
                    .short('1')
                    .help("1행을 표시하지 않음")
                    .action(ArgAction::SetFalse),
            )
            .arg(
                Arg::new("show_col2")
                    .short('2')
                    .help("2행을 표시하지 않음")
                    .action(ArgAction::SetFalse),
            )
            .arg(
                Arg::new("show_col3")
                    .short('3')
                    .help("3행을 표시하지 않음")
                    .action(ArgAction::SetFalse),
            )
            .arg(
                Arg::new("insensitive")
                    .short('i')
                    .help("대소문자를 구분하지 않음")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("delimiter")
                    .short('d')
                    .long("output-delimiter")
                    .value_name("DELIM")
                    .help("행간 구분 문자")
                    .default_value("\t"),
            )
            .arg(
                Arg::new("derive_mode")
                    .long("derive_mode")
                    .hide(true)
                    .help("`derive_mode`를 활성화하는 디버그용 플래그")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Args {
            // `.required(true)`이므로 `unwrap`을 사용할 수 있다.
            file1: matches.get_one("file1").cloned().unwrap(),
            // `.required(true)`이므로 `unwrap`을 사용할 수 있다.
            file2: matches.get_one("file2").cloned().unwrap(),
            show_col1: matches.get_flag("show_col1"),
            show_col2: matches.get_flag("show_col2"),
            show_col3: matches.get_flag("show_col3"),
            insensitive: matches.get_flag("insensitive"),
            // `.default_value("\t")`이므로 `unwrap`을 사용할 수 있다.
            delimiter: matches.get_one("delimiter").cloned().unwrap(),
            derive_mode: matches.get_flag("derive_mode"),
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        let file1 = self.file1.as_ref();
        let file2 = self.file2.as_ref();

        if file1 == "-" && file2 == "-" {
            anyhow::bail!("Both input files cannot be STDIN (\"-\")");
        }

        // 파일명을 전달하면 `Box<dyn BufRead>`를 반환하는 클로저이다.
        let get_bufread =
            |filename: &str| open(filename).map_err(|e| anyhow::anyhow!("{filename}: {e}"));
        let fh1 = get_bufread(file1)?;
        let fh2 = get_bufread(file2)?;

        let line_iter = LineIter::new(fh1, fh2, self.insensitive);

        let delim1 = if self.show_col1 {
            self.delimiter.as_str()
        } else {
            ""
        };
        let delim2 = if self.show_col2 {
            self.delimiter.as_str()
        } else {
            ""
        };

        line_iter.into_iter().try_for_each(|line_result| {
            match line_result? {
                LinePosition::First(st) => {
                    if self.show_col1 {
                        println!("{st}");
                    }
                }
                LinePosition::Second(st) => {
                    if self.show_col2 {
                        println!("{delim1}{st}");
                    }
                }
                LinePosition::Both(st) => {
                    if self.show_col3 {
                        println!("{delim1}{delim2}{st}");
                    }
                }
            };

            Result::<(), io::Error>::Ok(())
        })?;

        Ok(())
    }
}
