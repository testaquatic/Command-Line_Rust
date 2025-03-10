use args::Args;
use chrono::{Datelike, Local, NaiveDate};
use date::{format_month, parse_month};
use itertools::{Itertools, multizip};

mod args;
mod date;

pub fn run() -> Result<(), anyhow::Error> {
    let today = Local::now().date_naive();
    let args = Args::parse();
    let mut month = args.month.map(|month| parse_month(&month)).transpose()?;
    let mut year = args.year;

    if args.show_current_year {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }
    let year = year.unwrap_or(today.year());

    if let Some(month) = month {
        print_month(year, month, today)?;
    } else {
        print_year(year, today)?;
    }

    Ok(())
}

pub fn print_month(year: i32, month: u32, today: NaiveDate) -> Result<(), anyhow::Error> {
    let month = format_month(year, month, true, today)?;
    month.into_iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub fn print_year(year: i32, today: NaiveDate) -> Result<(), anyhow::Error> {
    println!("{:^66}", year);
    println!();
    let months = (1..=12)
        .map(|month| format_month(year, month, false, today))
        .chunks(3);
    months.into_iter().try_for_each(|mut months| {
        let month1 = months.next().unwrap()?;
        let month2 = months.next().unwrap()?;
        let month3 = months.next().unwrap()?;
        multizip((&month1, &month2, &month3))
            .for_each(|(m1, m2, m3)| println!("{}   {}   {}", m1, m2, m3));

        Result::<(), anyhow::Error>::Ok(())
    })
}
