use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;


use crate::parsers::nginx;
mod parsers;

#[derive(Debug)]
pub enum Error {
    InvalidArgs,
    ParsingError,
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

pub fn run(config: Config) -> Result<(), io::Error> {
    // Attempt to open the file
    let reader: Box<dyn BufRead> = match config.filename {
        Some(filename) => {
            let file = File::open(filename)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin()))
    };

    let mut x2: u32 = 0;
    let mut x3: u32 = 0;
    let mut x4: u32 = 0;
    let mut x5: u32 = 0;

    for line in reader.lines() {
        let logline = line.unwrap();
        let log = nginx::get_log_from_logline(&logline).unwrap();

        match log.status {
            200..=299 => { x2 += 1 }
            300..=399 => { x3 += 1 }
            400..=499 => { x4 += 1 }
            500..=599 => { x5 += 1 }
            _ => {}
        }
    }

    println!("2xx: {}", x2);
    println!("3xx: {}", x3);
    println!("4xx: {}", x4);
    println!("5xx: {}", x5);

    Ok(())
}

