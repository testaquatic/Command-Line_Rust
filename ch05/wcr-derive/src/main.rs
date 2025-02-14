use clap::Parser;
use wcr::Args;

fn main() {
    if let Err(e) = Args::parse().run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
