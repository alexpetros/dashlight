use std::env;
use dashlight::run;
use dashlight::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();

    run(config).expect("Program crashed, see stack trace");
}
