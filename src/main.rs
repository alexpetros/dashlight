use std::io::{self, BufRead};

use crate::parsers::nginx;
mod parsers;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let logline = line.unwrap();
        let log = nginx::get_log_from_logline(&logline);
        println!("{:?}", log);
    }
}
