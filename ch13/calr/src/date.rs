use std::fmt::Write;

use ansi_term::Style;
use chrono::{Datelike, Month, NaiveDate};

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn parse_month(month: &str) -> Result<u32, anyhow::Error> {
    if let Ok(m) = month.parse() {
        if (1..=12).contains(&m) {
            return Ok(m);
        }
        anyhow::bail!(r#"month "{}" not in the range 1 through 12"#, month);
    }

    let lower = month.to_lowercase();
    let mut match_months = MONTH_NAMES
        .iter()
        .enumerate()
        .filter(|&(_, month_name)| month_name.to_lowercase().starts_with(&lower))
        .map(|(i, _)| i + 1);

    if let Some(m) = match_months.next() {
        if match_months.next().is_none() {
            return Ok(m as u32);
        }
    }

    anyhow::bail!(r#"Invalid month "{}""#, month)
}

pub fn format_month(
    year: i32,
    month: u32,
    print_year: bool,
    today: NaiveDate,
) -> Result<Vec<String>, anyhow::Error> {
    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end = last_day_in_month(year, month)?;
    let today = if year == today.year() && month == today.month() {
        today.day()
    } else {
        // 날짜는 0이 될 수 없다.
        0
    };
    let mut month_vec = Vec::with_capacity(8);
    let head_line = format!(
        "{:^20}",
        if print_year {
            format!("{} {}", MONTH_NAMES[month as usize - 1], year)
        } else {
            MONTH_NAMES[month as usize - 1].to_string()
        }
    );
    month_vec.push(head_line);
    month_vec.push("Su Mo Tu We Th Fr Sa".to_string());

    let mut line = String::with_capacity(20);
    let start_day_weekday = start.weekday().num_days_from_sunday();
    (0..start_day_weekday).for_each(|_| line.push_str("   "));
    for (day, weekday) in (1..=end.day()).zip(start_day_weekday..) {
        if day == today {
            write!(
                line,
                "{}",
                Style::new().reverse().paint(format!("{:2}", day))
            )
            .unwrap();
        } else {
            write!(line, "{:2}", day).unwrap();
        }
        if weekday % 7 == 6 {
            month_vec.push(line);
            line = String::with_capacity(20);
        }
        if !line.is_empty() {
            line.push(' ');
        }
    }
    if line.len() < 20 {
        line.push_str(&" ".repeat(20 - line.len()));
    }
    month_vec.push(line);
    if month_vec.len() < 8 {
        month_vec.push(format!("{:20}", ""));
    }

    Ok(month_vec)
}

fn last_day_in_month(year: i32, month: u32) -> Result<NaiveDate, anyhow::Error> {
    Month::try_from(month as u8)?
        .num_days(year)
        .and_then(|day| NaiveDate::from_ymd_opt(year, month, day as u32))
        .ok_or(anyhow::anyhow!(
            "Not Valid: year: {}, month: {}",
            year,
            month
        ))
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    use crate::date::{format_month, last_day_in_month};

    use super::parse_month;

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "0" not in the range 1 through 12"#
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "13" not in the range 1 through 12"#
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"Invalid month "foo""#);
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_february = vec![
            "   February 2020    ",
            "Su Mo Tu We Th Fr Sa",
            "                   1",
            " 2  3  4  5  6  7  8",
            " 9 10 11 12 13 14 15",
            "16 17 18 19 20 21 22",
            "23 24 25 26 27 28 29",
            "                    ",
        ];
        assert_eq!(format_month(2020, 2, true, today).unwrap(), leap_february);

        let may = vec![
            "        May         ",
            "Su Mo Tu We Th Fr Sa",
            "                1  2",
            " 3  4  5  6  7  8  9",
            "10 11 12 13 14 15 16",
            "17 18 19 20 21 22 23",
            "24 25 26 27 28 29 30",
            "31                  ",
        ];
        assert_eq!(format_month(2020, 5, false, today).unwrap(), may);

        let april_hl = vec![
            "     April 2021     ",
            "Su Mo Tu We Th Fr Sa",
            "             1  2  3",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10",
            "11 12 13 14 15 16 17",
            "18 19 20 21 22 23 24",
            "25 26 27 28 29 30   ",
            "                    ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        assert_eq!(format_month(2021, 4, true, today).unwrap(), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 2).unwrap(),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 4).unwrap(),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }
}
