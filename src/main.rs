use dashlight::run;
use dashlight::Config;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let config = Config::new(&mut args);

    run(config).expect("Program crashed, see stack trace");
}
