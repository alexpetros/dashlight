use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use parsers::nginx;
use view::View;

mod parsers;
mod stats;
mod view;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgs,
    ParsingError,
}

#[derive(Debug)]
pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    // TODO: convert to OsString
    pub fn new(args: &[String]) -> Result<Config, Error> {
        if args.len() == 1 {
            return Ok(Config { filename: None });
        } else if args.len() == 2 {
            return Ok(Config {
                filename: Some(args[1].to_string()),
            });
        };

        Err(Error::InvalidArgs)
    }
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
        view.update(log);
        line.clear();
    }

    // Finish by printing the parsing results
    println!("{}", view);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_args_missing_name() {
        let args: Vec<String> = Vec::new();
        let config_result = Config::new(&args).unwrap_err();
        assert_eq!(config_result, Error::InvalidArgs);
    }

    #[test]
    fn no_args_no_filename() {
        let args = ["dashlight".to_string()];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.filename, None);
    }

    #[test]
    fn one_arg_filename() {
        let args = ["dashlight".to_string(), "access.log".into()];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.filename, Some("access.log".to_string()));
    }
}
