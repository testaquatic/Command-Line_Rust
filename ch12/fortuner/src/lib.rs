use args::Args;
use fortune::Fortunes;

mod args;
mod fortune;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // let args: Args = Parser::parse();
    let files = args.find_files()?;
    let fortunes = Fortunes::read_fortunes(&files)?;
    // 책과 같이 `Option`을 사용하는 것이 더 러스트다운 코드 같다.
    let mut prev_filename = None;

    match args.get_regex_from_pattern()? {
        Some(regex) => {
            fortunes.pick_fortune_by_regex(&regex).for_each(|fortune| {
                // 책에서 나온 코드에서 살짝 변경했다.
                if prev_filename.as_ref() != Some(&fortune.source) {
                    eprintln!(
                        "({})\n%",
                        // 존재하는 경로이므로 `unwrap`을 사용할 수 있다.
                        fortune.source.canonicalize().unwrap().display()
                    );

                    prev_filename = Some(fortune.source.clone());
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
