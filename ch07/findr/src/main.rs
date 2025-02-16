use std::process;

use clap::Parser;
use findr::args::Args;

fn main() {
    // 파서 패턴
    let args = <Args as Parser>::parse();
    // 빌더 패턴
    // let args = Args::parse();

    if let Err(e) = args.run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
