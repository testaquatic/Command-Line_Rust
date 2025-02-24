use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq)]
pub enum LinePosition {
    First(String),
    Second(String),
    Both(String),
}

pub struct LineIter {
    line1_iter: Box<dyn Iterator<Item = Result<String, io::Error>>>,
    line2_iter: Box<dyn Iterator<Item = Result<String, io::Error>>>,
    line1_next: Option<String>,
    line2_next: Option<String>,
    insensitive: bool,
}

impl LineIter {
    pub fn new(fh1: Box<dyn BufRead>, fh2: Box<dyn BufRead>, insensitive: bool) -> LineIter {
        LineIter {
            line1_iter: Box::new(fh1.lines().fuse()),
            line2_iter: Box::new(fh2.lines().fuse()),
            line1_next: None,
            line2_next: None,
            insensitive,
        }
    }

    /// 각 파일의 다음번 줄을 저장한다.
    fn fill_next(&mut self) -> Result<(), io::Error> {
        let case_change = |s: String| {
            if self.insensitive {
                s.to_lowercase()
            } else {
                s
            }
        };

        if self.line1_next.is_none() {
            self.line1_next = self.line1_iter.next().transpose()?.map(case_change);
        }
        if self.line2_next.is_none() {
            self.line2_next = self.line2_iter.next().transpose()?.map(case_change);
        }

        Ok(())
    }
}

impl Iterator for LineIter {
    type Item = Result<LinePosition, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = self.fill_next() {
            return Some(Err(e));
        }

        // 정말 편리한 `match`
        match (&self.line1_next, &self.line2_next) {
            // 아래는 `Some`인 것을 확인하므로 `unwrap`을 사용해도 안전하다
            (None, None) => None,
            (Some(line1_str), Some(line2_str)) => match line1_str.cmp(line2_str) {
                Ordering::Equal => {
                    self.line1_next = None;
                    Some(Ok(LinePosition::Both(self.line2_next.take().unwrap())))
                }
                Ordering::Less => Some(Ok(LinePosition::First(self.line1_next.take().unwrap()))),
                Ordering::Greater => {
                    Some(Ok(LinePosition::Second(self.line2_next.take().unwrap())))
                }
            },
            (Some(_), None) => Some(Ok(LinePosition::First(self.line1_next.take().unwrap()))),
            (None, Some(_)) => Some(Ok(LinePosition::Second(self.line2_next.take().unwrap()))),
        }
    }
}
