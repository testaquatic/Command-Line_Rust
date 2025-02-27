use args::Args;

mod args;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // let args: Args = Parser::parse();
    let pattern = args.get_regex_from_pattern()?;
    let files = args.find_files()?;
    println!("{files:?}");

    Ok(())
}
