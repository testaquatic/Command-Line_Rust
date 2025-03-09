use std::fmt::Write;

use ansi_term::Style;
use args::Args;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use month::MONTHS;

mod args;
mod month;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // let args = <Args as Parser>::parse();
    let today = Local::now().naive_l
    match args.month {
        None => print_year(args.year.unwrap(), today),
        Some(m) => todo!(),
    };

    Ok(())
}

fn print_year(year: i32, today: &NaiveDate) {
    println!("\n{:^70}\n", year);

    (0..=3).for_each(|quarter| {
        let quarter_months = (quarter * 3 + 1..=quarter * 3 + 3)
            .map(|month| format_month(year, month, false, today))
            .collect::<Vec<_>>();
        println!()
    });
}

// 주의: `year`와 `month`는 올바른 값이어야 한다.
fn format_month(year: i32, month: u32, print_year: bool, today: &NaiveDate) -> Vec<String> {
    // 이 프로그램에서는 파싱 단계에서 잘못된 year와 month를 검사하므로 unwrap을 사용해도 문제 없다.
    let start_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    // 일요일부터 0이 반환된다.
    let column = |day: &NaiveDate| {
        if day.weekday() == Weekday::Sun {
            0
        } else {
            day.weekday() as usize + 1
        }
    };

    let last_day = last_day_in_month(year, month);

    let mut out_strings = Vec::new();

    // `unwrap`을 사용해도 PC에서는 문제가 발생할 가능성이 매우 낮다.
    let head = format!(
        " {:^20}",
        format!(
            " {}{}",
            capitalize(MONTHS[month as usize - 1]),
            if print_year {
                format!(" {}", year)
            } else {
                "".to_string()
            }
        )
    );
    out_strings.push(head);
    out_strings.push(" Su Mo Tu We Th Fr Sa".to_string());

    let mut day_line = String::with_capacity(21);

    write!(&mut day_line, "{}", " ".repeat(column(&start_day) * 3)).unwrap();
    let mut day_line =
        start_day
            .iter_days()
            .filter(|day| day <= &last_day)
            .fold(day_line, |mut line, day| {
                if today == &day {
                    write!(
                        &mut line,
                        " {}",
                        Style::new().reverse().paint(format!("{:2}", day.day()))
                    )
                    .unwrap();
                } else {
                    write!(&mut line, " {:>2}", day.day()).unwrap();
                }
                if day.weekday() == Weekday::Sat {
                    out_strings.push(line);
                    line = String::with_capacity(24);
                }

                line
            });
    if !day_line.is_empty() {
        let remain_day = 6 - column(&last_day);
        write!(&mut day_line, "{}", " ".repeat(remain_day * 3)).unwrap();
        out_strings.push(day_line);
    }

    out_strings
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    (28..=31)
        .rev()
        .map(|day| NaiveDate::from_ymd_opt(year, month, day))
        .find(|day| day.is_some())
        // 모든 달은 28일에서 31일 이내므로 unwrap을 사용해도 문제 없다.
        .unwrap()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{format_month, last_day_in_month};

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_febrary = [
            "     February 2020   ",
            " Su Mo Tu We Th Fr Sa",
            "                    1",
            "  2  3  4  5  6  7  8",
            "  9 10 11 12 13 14 15",
            " 16 17 18 19 20 21 22",
            " 23 24 25 26 27 28 29",
        ];

        pretty_assertions::assert_eq!(format_month(2020, 2, true, &today), leap_febrary);

        let may = [
            "          May        ",
            " Su Mo Tu We Th Fr Sa",
            "                 1  2",
            "  3  4  5  6  7  8  9",
            " 10 11 12 13 14 15 16",
            " 17 18 19 20 21 22 23",
            " 24 25 26 27 28 29 30",
            " 31                  ",
        ];
        pretty_assertions::assert_eq!(format_month(2020, 5, false, &today), may);

        let april_h1 = [
            "      April 2021     ",
            " Su Mo Tu We Th Fr Sa",
            "              1  2  3",
            "  4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10",
            " 11 12 13 14 15 16 17",
            " 18 19 20 21 22 23 24",
            " 25 26 27 28 29 30   ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        pretty_assertions::assert_eq!(format_month(2021, 4, true, &today), april_h1);
    }

    #[test]
    fn test_last_day_in_month() {
        pretty_assertions::assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );
        pretty_assertions::assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );
        pretty_assertions::assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }
}
