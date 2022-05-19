use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;


use crate::parsers::nginx;
mod parsers;

#[derive(Debug)]
pub enum Error {
    InvalidArgs
}

pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Error> {
        if args.len() == 1 {
           return Ok(Config { filename: None });
        } else if args.len() == 2{
            return Ok(Config { filename: Some(args[1].clone()) });
        }

        Err(Error::InvalidArgs)
    }
}

pub fn run(config: Config) -> Result<(), Error> {
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
