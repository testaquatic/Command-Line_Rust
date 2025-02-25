use std::{fs::File, io::BufReader, str::FromStr};

use clap::{Arg, ArgAction, ArgGroup, Command, Parser};

use crate::{count_lines_bytes, get_start_index, process_line};

/// `tail`의 간단한 러스트 버전
#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct Args {
    /// 입력 파일(들)
    #[arg(required(true), num_args(1..), value_name("FILE"))]
    files: Vec<String>,

    /// 헤더를 표시하지 않음
    #[arg(short, long)]
    quiet: bool,
    #[command(flatten)]
    counter: Counter,
}

#[derive(Debug, clap::Args)]
struct Counter {
    /// 출력할 줄 수
    #[arg(short('n'), long, value_name("LINES"), default_value("10"))]
    lines: TakeValue,

    /// 출력할 바이트 수
    #[arg(short('c'), long, value_name("BYTES"), conflicts_with("lines"))]
    bytes: Option<TakeValue>,
}

/// 인수 파싱을 쉽게 하는 매크로.
/// 되도록이면 clap의 오류 메시지를 재사용하고 싶어서 만들었다.
macro_rules! parse_counter {
    ($counter:expr) => {
        |st: &str| {
            st.parse::<TakeValue>()
                .map_err(|e| anyhow::anyhow!("illegal {} count -- {e}", $counter))
        }
    };
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("tailr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("`tail`의 간단한 러스트 버전")
            .arg(
                Arg::new("files")
                    .required(true)
                    .num_args(1..)
                    .value_name("FILE")
                    .help("입력 파일(들)"),
            )
            .arg(
                Arg::new("lines")
                    .short('n')
                    .long("lines")
                    .value_name("LINES")
                    .default_value("10")
                    .help("출력할 줄 수")
                    .value_parser(parse_counter!("line")),
            )
            .arg(
                Arg::new("bytes")
                    .short('c')
                    .long("bytes")
                    .value_name("BYTES")
                    .help("출력할 바이트 수")
                    .value_parser(parse_counter!("byte")),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .action(ArgAction::SetTrue)
                    .help("헤더를 표시하지 않음"),
            )
            .group(ArgGroup::new("counter").args(["lines", "bytes"]))
            .get_matches();

        Args {
            // `.num_args(1..)`이므로 `unwrap`을 사용할 수 있다.
            files: matches.get_many("files").unwrap().cloned().collect(),
            counter: Counter {
                // `.default_value(10)`이므로 `unwrap`을 사용할 수 있다.
                lines: matches.get_one("lines").cloned().unwrap(),
                bytes: matches.get_one("bytes").cloned(),
            },

            quiet: matches.get_flag("quiet"),
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        self.files.iter().try_for_each(|filename| {
            let header_print =
                |start_idx: Option<u64>| !self.quiet && self.files.len() > 1 && start_idx.is_some();

            let print_header = |filename: &str, is_header_print: bool| {
                if is_header_print {
                    println!("==> {filename} <==");
                }
            };
            let print_footer = |filename: &str, is_header_print: bool| {
                // 이 블럭안에 진입하려면 `self.files.len()`은 최소한 1이다.
                // 따라서 `unwrap`을 사용할 수 있다.
                if is_header_print && self.files.last().unwrap() != filename {
                    println!();
                }
            };

            if let Some(bytes) = &self.counter.bytes {
                unimplemented!()
            } else {
                let mut b_f = BufReader::new(File::open(filename)?);
                let (total_lines, _) = count_lines_bytes(&mut b_f)?;
                let start_idx = get_start_index(&self.counter.lines, total_lines);
                let is_header_print = header_print(start_idx);
                print_header(filename, is_header_print);
                if let Err(e) = process_line(b_f, start_idx) {
                    eprintln!("{filename}: {e}");
                }
                print_footer(filename, is_header_print);
            }

            Result::<(), anyhow::Error>::Ok(())
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

impl FromStr for TakeValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+0" => Ok(TakeValue::PlusZero),
            _ => {
                let num = s.parse::<i64>().map_err(|_| anyhow::anyhow!("{s}"))?;
                if s.starts_with(['-', '+']) {
                    Ok(TakeValue::TakeNum(num))
                } else {
                    Ok(TakeValue::TakeNum(-num))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::args::TakeValue;

    #[test]
    fn test_parse_num() {
        // 모든 정수는 음수로 해석되어야 한다.
        let res = "3".parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        // 앞에 "+"를 붙이면 양수가 되어야 한다.
        let res = "+3".parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(3));

        // 명시적으로 "-"를 붙인 값은 음수가 되어야 한다.
        let res = "-3".parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        // 0은 0이다.
        let res = "0".parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(0));

        // +0은 특별하다
        let res = "+0".parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::PlusZero);

        // 경계 테스트
        let res = i64::MAX.to_string().parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN + 1));

        let res = (i64::MIN + 1).to_string().parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN + 1));

        let res = format!("+{}", i64::MAX).parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MAX));

        let res = i64::MIN.to_string().parse::<TakeValue>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN));

        // 부동소숫점 값은 유효하지 않다.
        let res = "3.14".parse::<TakeValue>();
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), "3.14");

        // 정수가 아닌 문자열은 모두 유효하지 않다.
        let res = "abc".parse::<TakeValue>();
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), "abc");
    }
}
