use std::io::{self, BufRead, Seek, SeekFrom, Write};

use args::{Args, TakeValue};
use clap::Parser;

mod args;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    args.run()
}

pub fn run_derive() -> Result<(), anyhow::Error> {
    let args = <Args as Parser>::parse();
    args.run()
}

/// 책의 코드가 더 효율적으로 보이므로 교체한다.
fn count_lines_bytes<T: BufRead>(f: &mut T) -> Result<(i64, i64), io::Error> {
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();

    loop {
        let len = f.read_until(b'\n', &mut buf)?;
        if len == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += len as i64;
        buf.clear();
    }

    Ok((num_lines, num_bytes))
}

/// 코드를 명확하게 정리했다.
fn print_lines<T: BufRead + Seek, U: Write>(
    f: &mut T,
    start_idx: Option<u64>,
    writer: &mut U,
) -> Result<(), io::Error> {
    // 파일 디스크립터가 처음을 가리키도록 설정한다.
    f.seek(SeekFrom::Start(0))?;

    if let Some(start_index) = start_idx {
        // `start_idx`만큼의 줄을 소비한다.
        f.lines().take(start_index as usize).for_each(|_| {});

        // 0이 반환되면 EOF이다.
        // 파일 끝을 유지하기 위해서 `read_line`을 사용한다.
        let mut line = String::new();
        while f.read_line(&mut line)? != 0 {
            write!(writer, "{line}")?;
            line.clear();
        }
    }

    Ok(())
}

/// 책의 코드가 더 효율적이므로 변경한다.
/// `start_idx`는 파일의 크기를 넘어서면 안된다.
/// `get_start_index`를 사용해서 `start_idx`를 산출하면 문제를 방지할 수 있다.
fn print_bytes<T: BufRead + Seek, U: Write>(
    f: &mut T,
    start_idx: Option<u64>,
    writer: &mut U,
) -> Result<(), io::Error> {
    if let Some(start_idx) = start_idx {
        f.seek(SeekFrom::Start(start_idx))?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            write!(writer, "{}", String::from_utf8_lossy(&buffer))?;
        }
    }

    Ok(())
}

fn get_start_index(take_value: &TakeValue, total_lines: i64) -> Option<u64> {
    match take_value {
        // 모든 것을 표시한다.
        TakeValue::PlusZero => {
            // 전체 길이가 0이면 아무것도 인쇄하지 않는다.
            if total_lines == 0 {
                None
            // 처음부터 읽기 시작한다.
            } else {
                Some(0)
            }
        }
        // 일부만 표시한다.
        TakeValue::TakeNum(num) => match num.signum() {
            // 0이면 아무것도 표시하지 않는다.
            0 => None,
            // `num`이 양수이면 해당하는 줄부터 읽는다.
            1 => {
                // 전체 줄을 벗어났다.
                if *num > total_lines {
                    None
                // `num`이 0이거나 양수이다.
                // 0인 때는 `None`을 반환한다.
                } else {
                    (*num as u64).checked_sub(1)
                }
            }
            // `num`이 음수라면 파일의 끝부터 읽는다.
            // 길이와 더한 이후에 음수일 때는 `Some(0)`를 반환한다.
            -1 => u64::try_from(total_lines + num).ok().or(Some(0)),
            // 도달할리 없지만 러스트 컴파일러가 오류를 반환한다.
            _ => unreachable!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes(&mut BufReader::new(
            File::open("tests/inputs/one.txt").unwrap(),
        ));
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes(&mut BufReader::new(
            File::open("tests/inputs/twelve.txt").unwrap(),
        ));
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap(), (12, 63));
    }

    #[test]
    fn test_get_start_index() {
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::PlusZero, 0), None);
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::PlusZero, 1), Some(0));
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(0), 1), None);
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(1), 0), None);
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(2), 1), None);
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(1), 10), Some(0));
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(2), 10), Some(1));
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(3), 10), Some(2));

        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(-1), 10), Some(9));
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(-2), 10), Some(8));
        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(-3), 10), Some(7));

        pretty_assertions::assert_eq!(get_start_index(&TakeValue::TakeNum(-11), 10), Some(0));
    }
}
