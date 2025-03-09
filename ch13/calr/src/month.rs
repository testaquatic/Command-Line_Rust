use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Month(u32);

/// 1월은 0부터 시작한다.
pub const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

impl Month {
    pub fn new(month: u32) -> Self {
        Month(month)
    }
}

impl FromStr for Month {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let try_parse_num = s.parse::<u32>();
        if let Ok(num) = try_parse_num {
            if num > 0 && num < 13 {
                return Ok(Month(num));
            } else {
                anyhow::bail!("month \"{}\" not in the range 1 through 12", s);
            }
        }

        // 책과 거의 유사한 코드이다.
        let mut matches = MONTHS
            .iter()
            .enumerate()
            .filter(|&(_, month)| month.starts_with(s.to_lowercase().as_str()))
            .map(|(idx, _)| idx as u32 + 1);

        let maybe_month = matches.next();
        // `matches`의 길이가 0이거나 2이상일 때
        if maybe_month.is_none() || matches.next().is_some() {
            anyhow::bail!("Invalid month \"{}\"", s);
        }

        // `maybe_month`는 Some(u32)이므로 `unwrap`을 사용할 수 있다.
        Ok(Month(maybe_month.unwrap()))
    }
}

impl From<Month> for u32 {
    fn from(month: Month) -> Self {
        month.0
    }
}

impl From<u32> for Month {
    fn from(month: u32) -> Self {
        Month(month)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_month() {
        let res = "1".parse::<Month>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), Month(1));

        let res = "12".parse::<Month>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), Month(12));

        let res = "jan".parse::<Month>();
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), Month(1));

        let res = "0".parse::<Month>();
        assert!(res.is_err());
        pretty_assertions::assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = "13".parse::<Month>();
        assert!(res.is_err());
        pretty_assertions::assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = "invalid".parse::<Month>();
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), "Invalid month \"invalid\"");
    }
}
