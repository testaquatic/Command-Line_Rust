use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use args::Args;

mod args;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    args.run()?;
    Ok(())
}

pub fn run_derive() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
