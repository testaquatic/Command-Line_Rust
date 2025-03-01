use std::path::PathBuf;

use args::Args;
use fortune::Fortunes;

mod args;
mod fortune;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // let args: Args = Parser::parse();
    let files = args.find_files()?;
    let fortunes = Fortunes::read_fortunes(&files)?;
    let mut prev_filename = PathBuf::new();

    match args.get_regex_from_pattern()? {
        Some(regex) => {
            fortunes
                .pick_fortune_by_regex(&regex)
                .into_iter()
                .for_each(|fortune| {
                    if files.len() > 1 && prev_filename != fortune.source {
                        eprintln!(
                            "({})\n%",
                            // 존재하는 경로이므로 `unwrap`을 사용할 수 있다.
                            fortune.source.canonicalize().unwrap().to_string_lossy()
                        );
                        prev_filename = fortune.source.clone();
                    }
                    println!("{}\n%", fortune.text);
                });
        }
        None => match fortunes.pick_fortune(args.seed) {
            Some(fortune) => println!("{}", fortune),
            None => println!("No fortunes found"),
        },
    }

    Ok(())
}
