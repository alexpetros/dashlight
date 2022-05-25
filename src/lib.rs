use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use config::Config;
use parsers::nginx;
use view::View;

pub mod config;

mod parsers;
mod stats;
mod view;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgs,
    ParsingError,
}

pub fn run(config: Config) -> Result<(), io::Error> {
    // Attempt to open the file if on was provided, STDIN otherwise
    let mut reader: Box<dyn BufRead> = match config.filename {
        Some(filename) => {
            let file = File::open(filename)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut view = View::new();
    let mut line = String::new();

    // Keep reading lines until we reach a line with 0 bytes
    while reader.read_line(&mut line).unwrap() > 0 {
        let logline = &line;
        let log = nginx::get_log_from_logline(&logline).unwrap();
        if config.mode == config::Mode::WATCH {
            view.update(log);
        } else {
            println!("{}", log);
        }

        line.clear();
    }

    // Finish by printing the parsing results
    if config.mode == config::Mode::WATCH {
        println!("{}", view);
    };

    Ok(())
}
