use grepr::args::Args;

fn main() {
    // 빌더 패턴
    let args = Args::parse();
    // 파생 패턴
    // let args = <Args as clap::Parser>::parse();

    if let Err(e) = args.run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
