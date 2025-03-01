use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use rand::{
    RngCore, SeedableRng,
    distr::{Distribution, slice::Choose},
    rngs::StdRng,
};
use regex::Regex;

#[derive(Debug)]
pub struct Fortune {
    pub source: PathBuf,
    pub text: String,
}

pub struct Fortunes(Vec<Fortune>);

impl Fortunes {
    pub fn read_fortunes(paths: &[PathBuf]) -> Result<Fortunes, anyhow::Error> {
        let mut string_buf = Vec::new();
        let mut fortunes = Fortunes(Vec::new());
        paths.iter().try_for_each(|path| {
            let mut f = BufReader::new(
                File::open(path).map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?,
            );

            // EOF를 만나면 0이 반환된다.
            while f.read_until(b'%', &mut string_buf)? != 0 {
                let text = String::from_utf8_lossy(&mut string_buf);
                // 조기에 `String`을 생성한다.
                // .read_until()은 구분자를 포함한다.
                let text_trim = text.trim_end_matches('%').trim().to_string();
                // 나중의 버그를 막기 위해서 빠르게 정리한다.
                string_buf.clear();
                if text_trim.is_empty() {
                    continue;
                }
                if !path.is_file() {
                    anyhow::bail!("{} is not a file!", path.display());
                }

                let fortune = Fortune {
                    source: path.to_owned(),
                    text: text_trim.to_string(),
                };
                fortunes.0.push(fortune);
            }

            Result::<(), anyhow::Error>::Ok(())
        })?;

        Ok(fortunes)
    }

    /// `Fortunes`에서 `String`을 무작위로 선택한다.  
    /// [https://rust-random.github.io/rand/rand/distr/slice/struct.Choose.html](https://rust-random.github.io/rand/rand/distr/slice/struct.Choose.html)의 코드를 참조했다.
    pub fn pick_fortune(&self, seed: Option<u64>) -> Option<String> {
        let mut rng: Box<dyn RngCore> = match seed {
            Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
            None => Box::new(rand::rng()),
        };

        let choose = Choose::new(&self.0).ok()?.sample(rng.as_mut());

        Some(choose.text.to_string())
    }

    pub fn pick_fortune_by_regex(&self, regex: &Regex) -> impl Iterator<Item = &Fortune> {
        self.0
            .iter()
            .filter(move |fortune| regex.is_match(&fortune.text))
    }
}

#[cfg(test)]
mod tests {

    use crate::fortune::Fortune;

    use super::Fortunes;

    #[test]
    fn test_read_fortunes() {
        // 입력 파일이 하나이다.
        let res = Fortunes::read_fortunes(&["./tests/inputs/jokes".into()]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            pretty_assertions::assert_eq!(fortunes.0.len(), 6);
            pretty_assertions::assert_eq!(
                fortunes.0.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
        }

        // 입력 파일이 여러개이다.
        let res = Fortunes::read_fortunes(&[
            "./tests/inputs/jokes".into(),
            "./tests/inputs/quotes".into(),
        ]);
        assert!(res.is_ok());
        pretty_assertions::assert_eq!(res.unwrap().0.len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = Fortunes(vec![
            Fortune {
                source: "fortunes".into(),
                text: "You cannot achieve the impossible without \
                        attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".into(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".into(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ]);

        // Pick a fortune with a seed
        pretty_assertions::assert_eq!(
            fortunes.pick_fortune(Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
