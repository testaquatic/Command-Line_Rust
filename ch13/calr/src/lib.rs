use args::Args;
use clap::Parser;

mod args;

pub fn run() {
    //let args = Args::parse();
    let args = <Args as Parser>::parse();
    println!("{:#?}", args);
}
