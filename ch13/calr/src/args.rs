use clap::{Arg, Command, value_parser};

#[derive(Debug)]
pub struct Args {
    pub year: Option<i32>,
    pub month: Option<String>,
    pub show_current_year: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("calr")
            .version("0.1.0")
            .author("TestAquatic")
            .about("간단한 러스트 버전 `cal`")
            .arg(
                Arg::new("year")
                    .value_name("YEAR")
                    .help("연도 (1-9999)")
                    .value_parser(value_parser!(i32).range(1..=9999)),
            )
            .arg(
                Arg::new("month")
                    .short('m')
                    .value_name("MONTH")
                    .help("달의 영문 이름이나 숫자 (1-12)"),
            )
            .arg(
                Arg::new("show_current_year")
                    .short('y')
                    .long("year")
                    .action(clap::ArgAction::SetTrue)
                    .help("현재 연도 표시")
                    .conflicts_with_all(["year", "month"]),
            )
            .get_matches();

        Args {
            year: matches.get_one("year").cloned(),
            month: matches.get_one("month").cloned(),
            show_current_year: matches.get_flag("show_current_year"),
        }
    }
}
