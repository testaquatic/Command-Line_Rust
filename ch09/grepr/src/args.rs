use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command, Parser};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

use crate::open;

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
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.insensitive)
            .build()
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        let pattern = self
            .make_regex()
            // 잘못된 패턴은 회복할 수 없는 오류이다.
            .map_err(|_| anyhow::anyhow!(r#"Invalid pattern "{}""#, self.pattern))?;

        self.get_bufreader_iter().try_for_each(|file_result| {
            let (line_result, filename) = match file_result {
                // 파일을 찾을 수 없을 때는 오류를 인쇄하고 넘어간다.
                Err(e) => {
                    eprintln!("{e}");
                    return Ok(());
                }
                Ok((b_reader, filename)) => (find_lines(b_reader, &pattern, self.invert), filename),
            };

            let finds = match line_result {
                // 회복할 수 있는 오류이다.
                Err(e) => {
                    eprintln!("{e}");
                    return Ok(());
                }
                Ok(finds) => finds,
            };

            let print_filename = || {
                if self.files.len() > 1 || self.recursive {
                    print!("{}:", filename.display());
                }
            };

            if self.count {
                print_filename();
                println!("{}", finds.len());
                return Ok(());
            }

            finds.iter().for_each(|find| {
                print_filename();
                print!("{}", find);
            });

            Result::<(), anyhow::Error>::Ok(())
        })?;

        Ok(())
    }

    fn get_file_list(&self) -> Vec<Result<PathBuf, anyhow::Error>> {
        let mut v = Vec::with_capacity(self.files.len());

        // -r 일 때
        if self.recursive {
            self.files.iter().for_each(|file| {
                let file_path = Path::new(file);
                WalkDir::new(file_path)
                    .into_iter()
                    // 파일이 아닌 것은 제거한다.
                    .flatten()
                    .filter(|entry| entry.file_type().is_file())
                    .for_each(|entry| v.push(Ok(entry.path().to_path_buf())))
            });

            return v;
            // -r이 없을 때
        }

        self.files.iter().for_each(|file| {
            let file_path = Path::new(file);
            // 먼저 STDIN을 처리한다.
            if file_path == Path::new("-") {
                v.push(Ok(file_path.to_path_buf()));
                return;
            }
            match file_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        v.push(Ok(file_path.to_path_buf()));
                    }
                    // 파일이 디렉터리일 때
                    else if metadata.is_dir() {
                        v.push(Err(anyhow::anyhow!(
                            "{} is a directory",
                            file_path.display()
                        )));
                    }
                    // 둘 다 아닐 때
                    else {
                        v.push(Err(anyhow::anyhow!(
                            "{} is not a file or directory",
                            file_path.display()
                        )));
                    }
                }
                // 메타데이터를 얻을 수 없을 때
                Err(e) => {
                    v.push(Err(anyhow::anyhow!("{}: {e}", file_path.display())));
                }
            }
        });

        v
    }

    /// `BufRead`의 이터레이터를 산출한다.
    fn get_bufreader_iter(
        &self,
    ) -> impl Iterator<Item = Result<(Box<dyn BufRead>, PathBuf), anyhow::Error>> {
        self.get_file_list().into_iter().map(|file| {
            let filename = file?;
            Ok((open(&filename)?, filename))
        })
    }
}

fn find_lines(
    mut file: impl BufRead,
    pattern: &Regex,
    invert: bool,
) -> Result<Vec<String>, anyhow::Error> {
    let mut line = String::new();
    let mut finds = Vec::new();

    loop {
        // EOF를 만나면 루프에서 빠져 나온다.
        if file.read_line(&mut line)? == 0 {
            break;
        }

        // `true && false` || `false && true` 일 때
        if pattern.is_match(&line) ^ invert {
            finds.push(line.clone());
        }

        line.clear();
    }

    Ok(finds)
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use rand::distr::{Alphanumeric, SampleString};
    use regex::{Regex, RegexBuilder};

    use super::{Args, find_lines};
    const PATH: &'static str = "./tests/inputs/";

    fn default_args() -> Args {
        Args {
            pattern: String::default(),
            files: Vec::new(),
            insensitive: false,
            recursive: false,
            count: false,
            invert: false,
        }
    }

    #[test]
    fn test_find_files() {
        // 파일의 존재 여부를 확인한다.
        let files = Args {
            files: vec![format!("{PATH}fox.txt")],
            ..default_args()
        }
        .get_file_list();
        pretty_assertions::assert_eq!(files.len(), 1);
        pretty_assertions::assert_eq!(
            files[0].as_ref().unwrap().to_string_lossy(),
            format!("{}fox.txt", PATH),
        );

        // 디렉터리 + 재귀옵션 없음
        let files = Args {
            files: vec![format!("{PATH}")],
            ..default_args()
        }
        .get_file_list();
        pretty_assertions::assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            pretty_assertions::assert_eq!(e.to_string(), format!("{} is a directory", PATH));
        }

        // 재귀적으로 네개의 파일을 찾는지 확인한다.
        let mut files = Args {
            files: vec![format!("{PATH}")],
            recursive: true,
            ..default_args()
        }
        .get_file_list()
        .iter()
        .map(|r| r.as_ref().unwrap().to_string_lossy().replace("\\", "/"))
        .collect::<Vec<_>>();
        files.sort();
        pretty_assertions::assert_eq!(files.len(), 4);
        pretty_assertions::assert_eq!(
            files,
            vec![
                format!("{PATH}bustle.txt"),
                format!("{PATH}empty.txt"),
                format!("{PATH}fox.txt"),
                format!("{PATH}nobody.txt")
            ]
        );

        // 존재하지 않는 파일
        let bad = Alphanumeric.sample_string(&mut rand::rng(), 7);
        let files = Args {
            files: vec![bad],
            ..default_args()
        }
        .get_file_list();
        pretty_assertions::assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // 패턴 _or_은 "Lorem" 한 줄을 매칭한다.
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        pretty_assertions::assert_eq!(matches.unwrap(), vec!["Lorem\n"]);

        // 뒤집기가 설정되면 이 함수는 다른 두 줄을 매칭한다.
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        pretty_assertions::assert_eq!(matches.unwrap(), vec!["Ipsum\r\n", "DOLOR"]);

        // 대소문자를 구문하지 않는 정규식이다.
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // 두줄을 매칭해야 한다.
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        pretty_assertions::assert_eq!(matches.unwrap(), vec!["Lorem\n", "DOLOR"]);

        // 뒤집기가 설정되면 남은 한줄은 매칭한다.
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        pretty_assertions::assert_eq!(matches.unwrap(), vec!["Ipsum\r\n"]);
    }
}
