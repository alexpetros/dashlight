use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::io;


use crate::parsers::nginx;
mod parsers;

pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {
           return Ok(Config { filename: None });
        } else if args.len() == 2{
            return Ok(Config { filename: Some(args[1].clone()) });
        }

        Err("Usage: {} [FILE] or {} for stdin")
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let reader: Box<dyn BufRead> = match config.filename {
        Some(filename) => {
            let file = File::open(filename).unwrap();
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin()))
    };

    for line in reader.lines() {
        let logline = line.unwrap();
        let log = nginx::get_log_from_logline(&logline);
        println!("{:?}", log);
    }

    Ok(())
}
