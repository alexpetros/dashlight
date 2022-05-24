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

fn find_flag_and_remove(args: &mut Vec<String>, flag: &'static str) -> Option<String> {
    args.iter()
        .position(|x| x == flag)
        .map(|index| args.remove(index))
}

fn find_named_and_remove(args: &mut Vec<String>, flag: &'static str) -> Option<String> {
    args.iter().position(|x| x == flag).map(|index| {
        // Verify that a value exists after the flag, so we can panic with our own message
        // Note that this does not check if the next value is a flag;
        // Technically starting with a dash is a valid filename
        args.get(index + 1)
            .unwrap_or_else(|| panic!("Missing value after {}", flag));
        // Draing the flag and the value after it, then return that value
        args.drain(index..index + 2).nth(1).unwrap()
    })
}

#[derive(Debug)]
pub struct Config {
    pub filename: Option<String>,
    pub quiet: bool,
}

impl Config {
    // TODO: convert to OsString
    pub fn new(mut args: Vec<String>) -> Config {
        let filename = find_named_and_remove(&mut args, "-f");
        let quiet = find_flag_and_remove(&mut args, "-q").is_some();
        return Config { filename, quiet };
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
    fn no_args_read_stdin() {
        let args = vec!["dashlight".to_string()];
        let config = Config::new(args);
        assert_eq!(config.filename, None);
    }

    #[test]
    fn one_arg_read_filename() {
        let args = vec!["dashlight".to_string(), "-f".into(), "access.log".into()];
        let config = Config::new(args);
        assert_eq!(config.filename, Some("access.log".to_string()));
    }

    #[test]
    #[should_panic(expected = "Missing value after -f")]
    fn invalid_args_missing_filename_after_f() {
        let args = vec!["dashlight".to_string(), "-f".into()];
        Config::new(args);
    }
}
