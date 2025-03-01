use std::{ffi::OsStr, io, path::PathBuf};

use clap::{Arg, ArgAction, Command, Parser, value_parser};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(name = "fortuner", author, version, about)]
pub struct Args {
    /// 입력 파일이나 디렉터리
    #[arg(value_name = "FILE", num_args = 1.., required = true, )]
    sources: Vec<String>,

    /// 패턴
    #[arg(short = 'm', long, value_name = "PATTERN")]
    pub pattern: Option<String>,

    /// 대소문자 구분 없이 매칭
    #[arg(short, long, action = ArgAction::SetTrue)]
    insensitive: bool,

    /// 랜덤 시드
    #[arg(short, long, value_name = "SEED", value_parser = value_parser!(u64))]
    pub seed: Option<u64>,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("fortuner")
            .version("0.1.0")
            .author("TestAquatic")
            .about("`fortune`의 간단한 러스트 버전")
            .arg(
                Arg::new("sources")
                    .value_name("FILE")
                    .num_args(1..)
                    .help("입력 파일이나 디렉터리")
                    .required(true),
            )
            .arg(
                Arg::new("pattern")
                    .short('m')
                    .long("pattern")
                    .value_name("PATTERN")
                    .help("패턴"),
            )
            .arg(
                Arg::new("insensitive")
                    .short('i')
                    .long("insensitive")
                    .action(ArgAction::SetTrue)
                    .help("대소문자 구분 없이 매칭"),
            )
            .arg(
                Arg::new("seed")
                    .short('s')
                    .long("seed")
                    .value_name("SEED")
                    .help("랜덤 시드")
                    .value_parser(value_parser!(u64)),
            )
            .get_matches();

        Args {
            // `.num_args(1..)`이므로 `unwrap()`을 사용할 수 있다.
            sources: matches.get_many("sources").unwrap().cloned().collect(),
            pattern: matches.get_one("pattern").cloned(),
            insensitive: matches.get_flag("insensitive"),
            seed: matches.get_one("seed").cloned(),
        }
    }

    pub fn get_regex_from_pattern(&self) -> Result<Option<Regex>, anyhow::Error> {
        self.pattern
            .as_ref()
            .map(|s| {
                RegexBuilder::new(s.as_str())
                    .case_insensitive(self.insensitive)
                    .build()
                    .map_err(|_| anyhow::anyhow!(r#"Invalid --pattern "{s}""#))
            })
            .transpose()
    }

    pub fn find_files(&self) -> Result<Vec<PathBuf>, io::Error> {
        let mut files_vec = self
            .sources
            .iter()
            .flat_map(|source| {
                WalkDir::new(source)
                    .into_iter()
                    .map(|entry_result| Result::<PathBuf, io::Error>::Ok(entry_result?.into_path()))
            })
            .filter(|path_result| {
                if let Ok(path) = path_result {
                    if path.extension() == Some(OsStr::new("dat")) || path.is_dir() {
                        return false;
                    }
                }

                true
            })
            // `BTreeSet`보다 책의 `Vec`이 더 효율적인 것 같다.
            .collect::<Result<Vec<_>, _>>()?;

        files_vec.sort();
        files_vec.dedup();

        Ok(files_vec)
    }
}

#[cfg(test)]
mod tests {

    use super::Args;

    const INPUT_DIR: &str = "./tests/inputs";

    /// 편의를 위한 함수이다.
    fn get_test_args(sources: Vec<String>) -> Args {
        Args {
            sources: sources,
            pattern: None,
            insensitive: false,
            seed: None,
        }
    }

    #[test]
    fn test_find_files() {
        // 존재하는 파일은 찾아야 한다.
        let res = get_test_args(vec![format!("{INPUT_DIR}/jokes")]).find_files();
        assert!(res.is_ok());
        let files = res.unwrap();
        pretty_assertions::assert_eq!(files.len(), 1);
        pretty_assertions::assert_eq!(files[0].to_str().unwrap(), format!("{INPUT_DIR}/jokes"));

        // 존재하지 않는 파일은 오류가 발생해야 한다.
        let res = get_test_args(vec!["/path/does/not/exist".to_string()]).find_files();
        assert!(res.is_err());

        // `.dat`을 찾아서는 안된다.
        let res = get_test_args(vec![INPUT_DIR.to_string()]).find_files();
        assert!(res.is_ok());
        let files = res.unwrap();

        // 파일의 개수와 순서를 확인한다.
        pretty_assertions::assert_eq!(files.len(), 5);
        let first = files[0].display().to_string();
        let last = files.last().unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        assert!(last.contains("quotes"));

        // 여러개의 소스를 확인한다.
        let res = get_test_args(vec![
            format!("{INPUT_DIR}/ascii-art"),
            format!("{INPUT_DIR}/jokes"),
            format!("{INPUT_DIR}/jokes"),
        ])
        .find_files();
        assert!(res.is_ok());
        let files = res.unwrap();
        pretty_assertions::assert_eq!(files.len(), 2);
        if let Some(path) = files.first().unwrap().file_name() {
            pretty_assertions::assert_eq!(path.to_str().unwrap(), "ascii-art");
        }
        if let Some(path) = files.last().unwrap().file_name() {
            pretty_assertions::assert_eq!(path.to_str().unwrap(), "jokes");
        }
    }
}
