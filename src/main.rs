use dashlight::config::Config;
use dashlight::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);

    run(config).expect("Program crashed, see stack trace");
}
