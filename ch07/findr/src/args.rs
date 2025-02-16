use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// `find`의 러스트 버전
pub struct Args {
    /// 검색 경로(들)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
    /// 이름
    #[arg(short, long("name"), value_name = "NAME", value_parser(Regex::new), action(ArgAction::Append), num_args(0..))]
    names: Vec<Regex>,
    /// 항목 유형
    #[arg(short('t'), long("type"), value_name = "TYPE", value_parser(clap::value_parser!(EntryType)), action(ArgAction::Append), num_args(0..))]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d").alias("dir"),
            EntryType::File => PossibleValue::new("f").alias("file"),
            EntryType::Link => PossibleValue::new("l").alias("link"),
        })
    }
}

impl EntryType {
    // 엔트리의 유형이 일치하는지 확인한다.
    fn check_file(&self, entry: &DirEntry) -> bool {
        match self {
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
            EntryType::Link => entry.file_type().is_symlink(),
        }
    }
}

impl Args {
    pub fn parse() -> Self {
        let matches = clap::Command::new("find")
            .about("러스트 버전 `find`")
            .author("TestAquatic")
            .version("0.1.0")
            .arg(
                clap::Arg::new("paths")
                    .value_name("PATH")
                    .num_args(0..)
                    .help("찾을 경로")
                    .default_value("."),
            )
            .arg(
                clap::Arg::new("names")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .num_args(0..)
                    .help("이름")
                    .value_parser(Regex::new)
                    .action(ArgAction::Append),
            )
            .arg(
                clap::Arg::new("types")
                    .short('t')
                    .long("type")
                    .value_name("TYPE")
                    .num_args(0..)
                    .help("유형")
                    .value_parser(clap::value_parser!(EntryType))
                    .action(ArgAction::Append),
            )
            .get_matches();

        let paths = matches
            .get_many("paths")
            // `paths`는 1..이므로 unwrap을 사용할 수 있다.
            .unwrap()
            .cloned()
            .collect();

        let names = matches
            .get_many("names")
            // `ValuesRef`의 기본값은 빈 이터레이터이다.
            // https://docs.rs/clap/latest/clap/parser/struct.ValuesRef.html
            .unwrap_or_default()
            .cloned()
            .collect();

        let entry_types = matches
            .get_many("types")
            // `ValuesRef`의 기본값은 빈 이터레이터이다.
            // https://docs.rs/clap/latest/clap/parser/struct.ValuesRef.html
            .unwrap_or_default()
            .cloned()
            .collect();

        Self {
            paths,
            names,
            entry_types,
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        self.paths.iter().for_each(|path| {
            WalkDir::new(path)
                .into_iter()
                .for_each(|entry| match entry {
                    Err(e) => eprintln!("{e}"),
                    Ok(entry) => {
                        let names_match =
                            self.names.iter().any(|name| check_file_name(&entry, name))
                            // 벡터가 비어 있을 때는 `true`이다.
                                || self.names.is_empty();
                        let types_match = self
                            .entry_types
                            .iter()
                            .any(|entry_type| entry_type.check_file(&entry))
                            // 벡터가 비어 있을 때는 `true`이다.
                            || self.entry_types.is_empty();
                        (names_match && types_match)
                            .then(|| println!("{}", entry.path().display()));
                    }
                })
        });

        Ok(())
    }
}

/// 파일 경로가 정규식과 일치하는지 확인한다.
fn check_file_name(entry: &DirEntry, name: &Regex) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    name.is_match(&file_name)
}
