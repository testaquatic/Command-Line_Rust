use cutr::args::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = args.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
