use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `echo`의 러스트 버전
struct Args {
    /// 입력 텍스트
    #[arg(required(true))]
    text: Vec<String>,

    /// 새 줄을 인쇄하지 않는다.
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    let line_end = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), line_end);
}
