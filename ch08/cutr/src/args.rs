mod arg_range;

use std::io::BufRead;

use arg_range::ArgRangeList;
use clap::{value_parser, Arg, ArgGroup, Command, Parser};

use crate::file::open;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `cut`의 러스트 버전
pub struct Args {
    /// 입력 파일(들)
    #[arg(value_name = "FILES", default_value = "-", num_args(0..))]
    files: Vec<String>,
    /// 구분 기호
    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,
    #[command(flatten)]
    extract: ArgExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true)]
struct ArgExtract {
    /// 선택한 필드
    #[arg(short, long, value_name = "FIELDS", value_parser(value_parser!(ArgRangeList)))]
    fields: Option<ArgRangeList>,
    /// 선택한 바이트
    #[arg(short, long, value_name = "BYTES", value_parser(value_parser!(ArgRangeList)))]
    bytes: Option<ArgRangeList>,
    /// 선택한 문자
    #[arg(short, long, value_name = "CHARS", value_parser(value_parser!(ArgRangeList)))]
    chars: Option<ArgRangeList>,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("cutr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("`cut`의 러스트 버전")
            .arg(
                Arg::new("files")
                    .value_name("FILES")
                    .num_args(0..)
                    .help("입력 파일(들)")
                    .default_value("-"),
            )
            .arg(
                Arg::new("delimiter")
                    .short('d')
                    .long("delimiter")
                    .value_name("DELIMITER")
                    .help("구분 문자")
                    .default_value("\t"),
            )
            .arg(
                Arg::new("fields")
                    .short('f')
                    .long("fields")
                    .value_name("FIELDS")
                    .value_parser(value_parser!(ArgRangeList))
                    .help("추출할 필드"),
            )
            .arg(
                Arg::new("bytes")
                    .short('b')
                    .long("bytes")
                    .value_name("BYTES")
                    .value_parser(value_parser!(ArgRangeList))
                    .help("추출할 바이트 범위"),
            )
            .arg(
                Arg::new("chars")
                    .short('c')
                    .long("chars")
                    .value_name("CHARS")
                    .value_parser(value_parser!(ArgRangeList))
                    .help("추출할 문자 범위"),
            )
            .group(
                ArgGroup::new("arg_extract")
                    .args(["fields", "bytes", "chars"])
                    .required(true), // `.multiple`의 기본값는 `false`이다.
                                     // .multiple(false)
            )
            .get_matches();

        // files는 기본값이 있으므로 `unwrap`을 사용할 수 있다.
        let files = matches.get_many("files").unwrap().cloned().collect();
        // delimiter는 기본값이 있으므로 `unwrap`을 사용할 수 있다.
        let delimiter = matches.get_one::<String>("delimiter").cloned().unwrap();

        // `ArgRangeList`로의 파싱은 `FromStr`을 구현했으므로 `clap`한테 떠넘긴다.
        let fields = matches.get_one("fields").cloned();
        let bytes = matches.get_one("bytes").cloned();
        let chars = matches.get_one("chars").cloned();
        let extract = ArgExtract {
            fields,
            bytes,
            chars,
        };

        Args {
            files,
            delimiter,
            extract,
        }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        // 문자열의 길이는 바이트열의 길이이므로 `as_bytes()`를 사용하지 않아도 된다.
        // 파싱한 `self.delimiter`를 점검한다.
        if self.delimiter.len() != 1 {
            anyhow::bail!(r#"--delim "{}" must be a single byte"#, self.delimiter);
        }

        self.files
            .iter()
            .filter_map(|filename| match open(filename) {
                Err(err) => {
                    eprintln!("{filename}: {err}");
                    None
                }
                Ok(file) => Some(file),
            })
            .try_for_each(|mut file| self.process_file_and_print(&mut file))
    }

    /// 파일을 처리하고 인쇄한다.
    fn process_file_and_print(&self, mut file: &mut dyn BufRead) -> Result<(), anyhow::Error> {
        if let Some(range) = &self.extract.fields {
            let mut reader = csv::ReaderBuilder::new()
                // `Args::run`에서 `self.delimiter`에 대한 검사를 하므로 인덱스를 사용해도 문제가 없다.
                .delimiter(self.delimiter.as_bytes()[0])
                .from_reader(&mut file);
            let print_string_vec = |vec: Vec<&str>| println!("{}", vec.join(&self.delimiter));

            let header_record = reader.headers()?;
            let header_string = range.extract_fields(header_record);
            print_string_vec(header_string);
            reader.records().try_for_each(|record| {
                let record = record?;
                let selected = range.extract_fields(&record);
                print_string_vec(selected);

                Result::<(), anyhow::Error>::Ok(())
            })?
        }

        file.lines().try_for_each(|line| {
            let line = line?;
            if let Some(range) = &self.extract.bytes {
                let selected = range.extract_bytes(&line);
                println!("{selected}");
            } else if let Some(range) = &self.extract.chars {
                let selected = range.extract_chars(&line);
                println!("{selected}");
            }

            Ok(())
        })
    }
}
