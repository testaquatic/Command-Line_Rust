use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub paths: Vec<String>,
    pub long: bool,
    pub show_hiden: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("lsr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("Rust version of `ls`")
            .arg(
                Arg::new("paths")
                    .value_name("PATH")
                    .num_args(0..)
                    .help("파일 또는 디렉토리")
                    .default_value("."),
            )
            .arg(
                Arg::new("long")
                    .short('l')
                    .long("long")
                    .help("상세 정보 출력")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("show_hiden")
                    .short('a')
                    .long("all")
                    .help("모든 파일 표시")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Args {
            // 기본값이 있으므로 `unwrap`을 사용할 수 있다.
            paths: matches.get_many("paths").unwrap().cloned().collect(),
            long: matches.get_flag("long"),
            show_hiden: matches.get_flag("show_hiden"),
        }
    }
}
