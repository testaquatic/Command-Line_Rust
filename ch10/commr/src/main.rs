use commr::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // derive 모드
    // run_derive().unwrap();
}
