use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
