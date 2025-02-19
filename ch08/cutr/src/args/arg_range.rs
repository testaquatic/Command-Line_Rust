use std::{error::Error, fmt, ops::Range, str::FromStr};

use csv::StringRecord;

#[derive(Debug, Clone)]
/// 추출할 부분의 리스트이다.
pub struct ArgRangeList {
    ranges: Vec<Range<usize>>,
}

impl ArgRangeList {
    pub fn extract_chars(&self, line: &str) -> String {
        self.ranges
            .iter()
            .map(|range| {
                line.chars()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect::<String>()
            })
            .collect()
    }

    pub fn extract_bytes(&self, line: &str) -> String {
        self.ranges
            .iter()
            .map(|range| {
                let selected = line
                    .bytes()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect::<Vec<_>>();
                String::from_utf8_lossy(&selected).to_string()
            })
            .collect()
    }

    pub fn extract_fields<'a>(&self, record: &'a StringRecord) -> Vec<&'a str> {
        self.ranges
            .iter()
            .flat_map(|range| {
                record
                    .iter()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

/// 문자열에서 리스트를 생성한다.
impl TryFrom<&str> for ArgRangeList {
    type Error = ArgRangeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ranges = s
            // 각 항목은 쉼표로 구분한다.
            .split(",")
            // 항목을 `ArgRange`로 변환한다.
            .map(|split| {
                let arg_range: ArgRange = split.try_into()?;
                Ok(arg_range.into())
            })
            // 벡터로 수집한다.
            .collect::<Result<_, _>>()?;

        Ok(ArgRangeList { ranges })
    }
}

/// `TryFrom<&str>`을 이미 구현했기 때문에 이용한다.
impl FromStr for ArgRangeList {
    type Err = ArgRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// 문자열을 `Range<usize>`로 변환하는 가교 역활하는 하는 타입이다.
/// `TryFrom<&str> for Range<usize>`는 러스트에서 고아규칙에 걸릴 것 같다.(시도 안해봄)
/// 그리고 약간의 복잡함을 감수하면 비용면에서 큰 차이가 없을 것 같다.
#[derive(Clone, Debug)]
struct ArgRange {
    start: usize,
    end: usize,
}

impl TryFrom<&str> for ArgRange {
    type Error = ArgRangeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split_value = s.split('-');

        let value_checker = |s_check: &str| {
            // 앞 부분이 '+'로 시작하지 않아야 한다.
            if s_check.starts_with('+') {
                return Err(ArgRangeError::FormatError(s.to_string()));
            }
            // 위치에 "0"은 사용할 수 없다.
            if s_check == "0" {
                return Err(ArgRangeError::FormatError(s_check.to_string()));
            }

            Ok(())
        };
        let str_parse = |s_parse: &str| {
            s_parse
                .parse::<usize>()
                .map_err(|_| ArgRangeError::FormatError(s.to_string()))
        };

        let start = match split_value.next() {
            None => return Err(ArgRangeError::FormatError(s.to_string())),
            Some(v) => {
                value_checker(v)?;
                str_parse(v)?
            }
        };

        let end = match split_value.next() {
            None => start,
            Some(v) => {
                value_checker(v)?;
                let end = str_parse(v)?;
                if start >= end {
                    return Err(ArgRangeError::ValueError((start, end)));
                }

                end
            }
        };

        // 값이 남아 있으면 안된다.
        if split_value.next().is_some() {
            return Err(ArgRangeError::FormatError(s.to_string()));
        }

        Ok(ArgRange { start, end })
    }
}

impl FromStr for ArgRange {
    type Err = ArgRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl From<ArgRange> for Range<usize> {
    fn from(value: ArgRange) -> Self {
        value.start - 1..value.end
    }
}

/// 파싱하는 과정에서 반환하는 오류이다.
#[derive(Debug, Clone)]
pub enum ArgRangeError {
    FormatError(String),
    ValueError((usize, usize)),
}

impl Error for ArgRangeError {}

impl fmt::Display for ArgRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgRangeError::FormatError(value) => write!(f, "illegal list value: \"{}\"", value),
            ArgRangeError::ValueError((start, end)) => {
                write!(
                    f,
                    "First number in range({}) must be lower than secound number ({})",
                    start, end
                )
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use std::{ops::Range, str::FromStr, vec};

    use csv::StringRecord;

    use crate::args::arg_range::ArgRangeList;

    #[test]
    fn test_parse_pos() {
        let format_err_string = |value: &str| format!("illegal list value: \"{}\"", value);

        // 빈 문자열은 오류다.
        assert!(ArgRangeList::from_str("").is_err());

        // 0은 오류이다.
        let v = "0";
        let res = ArgRangeList::from_str(&v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        let v = "0-1";
        let res = ArgRangeList::from_str(&v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string("0"));

        // 앞에 "+"가 오면 오류이다.
        let v = "+1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        let v = "+1-2";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        let v = "1-+2";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        // 수가 아닌 것은 전부 오류이다.
        let v = "a";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        let v = "1,a";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string("a"));

        let v = "1-a";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        let v = "a-1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), format_err_string(v));

        // 어딘가 좀 모자란 범위들
        let v = "-";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let v = ",";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let v = "1,";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let v = "1-";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let v = "1-1-1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let v = "1-1-a";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());

        let value_err_string = |start: usize, end: usize| {
            format!(
                "First number in range({}) must be lower than secound number ({})",
                start, end
            )
        };

        // 첫번째 수는 두 번째 수보다 작아야 한다.
        let v = "1-1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), value_err_string(1, 1));

        let v = "2-1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_err());
        pretty_assertions::assert_eq!(res.unwrap_err().to_string(), value_err_string(2, 1));

        // 다음은 전부 허용된다.
        let range_compare = |range: ArgRangeList, expected: Vec<Range<usize>>| {
            pretty_assertions::assert_eq!(range.ranges.len(), expected.len());
            pretty_assertions::assert_eq!(range.ranges, expected);
        };

        let v = "1";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..1]);

        let v = "01";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..1]);

        let v = "1,3";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..1, 2..3]);

        let v = "001,0004";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..1, 3..4]);

        let v = "1-3";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..3]);

        let v = "0001-03";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..3]);

        let v = "1,7,3-5";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let v = "15,19-20";
        let res = ArgRangeList::from_str(v);
        assert!(res.is_ok());
        range_compare(res.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_extract_chars() {
        let abc = "ábc";
        pretty_assertions::assert_eq!(ArgRangeList::from_str("1").unwrap().extract_chars(""), "");
        pretty_assertions::assert_eq!(ArgRangeList::from_str("1").unwrap().extract_chars(abc), "á");
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1,3").unwrap().extract_chars("ábc"),
            "ác"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1-3").unwrap().extract_chars("ábc"),
            "ábc"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("3,2").unwrap().extract_chars("ábc"),
            "cb"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1,2,5")
                .unwrap()
                .extract_chars("ábc"),
            "áb"
        );
    }

    #[test]
    fn test_extract_bytes() {
        let abc = "ábc";
        pretty_assertions::assert_eq!(ArgRangeList::from_str("1").unwrap().extract_bytes(abc), "�");
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1-2").unwrap().extract_bytes(abc),
            "á"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1-3").unwrap().extract_bytes(abc),
            "áb"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1-4").unwrap().extract_bytes(abc),
            abc
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("4,3").unwrap().extract_bytes(abc),
            "cb"
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1-2,6").unwrap().extract_bytes(abc),
            "á"
        );
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1").unwrap().extract_fields(&rec),
            &["Captain"]
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("2").unwrap().extract_fields(&rec),
            &["Sham"]
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1,3").unwrap().extract_fields(&rec),
            &["Captain", "12345"]
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("1,4").unwrap().extract_fields(&rec),
            &["Captain"]
        );
        pretty_assertions::assert_eq!(
            ArgRangeList::from_str("2,1").unwrap().extract_fields(&rec),
            &["Sham", "Captain"]
        );
    }
}
