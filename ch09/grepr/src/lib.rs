use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub mod args;

fn open(filename: impl AsRef<Path>) -> Result<Box<dyn BufRead>, anyhow::Error> {
    if filename.as_ref() == Path::new("-") {
        Ok(Box::new(BufReader::new(io::stdin().lock())))
    } else {
        Ok(Box::new(BufReader::new(
            File::open(filename.as_ref())
                .map_err(|e| anyhow::anyhow!("{}: {e}", filename.as_ref().display()))?,
        )))
    }
}
