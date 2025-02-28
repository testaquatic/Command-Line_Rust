use args::Args;
use fortune::Fortunes;

mod args;
mod fortune;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // let args: Args = Parser::parse();
    let files = args.find_files()?;
    let fortunes = Fortunes::read_fortunes(&files)?;

    match args.get_regex_from_pattern()? {
        Some(regex) => {
            fortunes.pick_fortune_by_regex(&regex).into_iter().try_for_each(|fortune| {
                print
            })?;
        }
        None => match fortunes.pick_fortune(args.seed) {
            Some(fortune) => println!("{}", fortune),
            None => println!("No fortunes found"),
        },
    }

    Ok(())
}
