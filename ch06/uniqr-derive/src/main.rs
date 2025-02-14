use clap::Parser;
use uniqr::args::Args;

fn main() {
    let args = <Args as Parser>::parse();
    if let Err(e) = args.run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
