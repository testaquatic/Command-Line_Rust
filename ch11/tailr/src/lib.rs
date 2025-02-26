use std::io::{self, BufRead, Read, Seek, SeekFrom};

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

fn count_lines_bytes<T: BufRead>(f: &mut T) -> Result<(i64, i64), io::Error> {
    f.bytes().try_fold((0, 0), |(lines, bytes), b_reuslt| {
        let b = b_reuslt?;
        Ok((lines + if b == b'\n' { 1 } else { 0 }, bytes + 1))
    })
}

fn print_lines<T: BufRead + Seek>(f: &mut T, start_idx: Option<u64>) -> Result<(), io::Error> {
    // 파일 디스크립터가 처음을 가리키도록 설정한다.
    f.seek(SeekFrom::Start(0))?;

    if let Some(start_index) = start_idx {
        let mut line = String::new();
        let mut loop_count = 0;

        // EOF를 만나면 .read_line()은 Ok(0)을 반환한다.
        while loop_count < start_index && f.read_line(&mut line)? != 0 {
            line.clear();
            loop_count += 1;
        }

        // 0이 반환되면 EOF이다.
        while f.read_line(&mut line)? != 0 {
            print!("{line}");
            line.clear();

            loop_count += 1;
        }
    }

    Ok(())
}

/// `start_idx`는 파일의 크기를 넘어서면 안된다.
/// `get_start_index`를 사용해서 `start_idx`를 산출하면 문제를 방지할 수 있다.
fn print_bytes<T: BufRead + Seek>(f: &mut T, start_idx: Option<u64>) -> Result<(), io::Error> {
    if let Some(start_idx) = start_idx {
        f.seek(SeekFrom::Start(start_idx))?;
    }

    let bytes = f.bytes().collect::<Result<Vec<u8>, io::Error>>()?;
    print!("{}", String::from_utf8_lossy(&bytes));

    Ok(())
}

fn get_start_index(take_value: &TakeValue, total_lines: i64) -> Option<u64> {
    match take_value {
        TakeValue::PlusZero => {
            if total_lines == 0 {
                None
            } else {
                Some(0)
            }
        }
        TakeValue::TakeNum(num) => match num {
            0 => None,
            &num if num > 0 => {
                if num > total_lines {
                    None
                } else {
                    (num as u64).checked_sub(1)
                }
            }
            &num if num < 0 => u64::try_from(total_lines + num).ok().or(Some(0)),
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
