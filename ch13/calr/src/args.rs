use clap::{Arg, ArgAction, ArgGroup, Command, Parser, value_parser};

#[derive(Debug, Parser)]
#[command(name = "calr", author, version, about)]
pub struct Args {
    /// 연도 (1-9999)
    #[arg(value_name = "YEAR", value_parser = value_parser!(i32).range(1..=9999))]
    year: Option<i32>,

    /// 달의 이름이나 숫자 (1-12)
    #[arg(value_name = "MONTH", short)]
    month: Option<String>,

    /// 올해를 모두 나타낸다.
    #[arg(short='y', long = "year", action = ArgAction::SetTrue, conflicts_with_all = ["month", "year"])]
    show_current_year: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("calr")
            .author("TestAquatic")
            .about("`cal`의 간단한 러스트 구현")
            .version("0.1.0")
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
                    .help("달의 이름이나 숫자 (1-12)"),
            )
            .arg(
                Arg::new("show_current_year")
                    .short('y')
                    .long("year")
                    .help("올해를 모두 나타낸다")
                    .conflicts_with_all(["month", "year"])
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Args {
            year: matches.get_one("year").cloned(),
            month: matches.get_one("month").cloned(),
            show_current_year: matches.get_flag("show_current_year"),
        }
    }
}
