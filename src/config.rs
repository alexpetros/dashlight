use crate::Error;
use std::process;

const USAGE_TEXT: &str = "Usage: dashlight [-f filename] [watch|convert]";
const HELP_TEXT: &str = r#"Usage: dashlight [-f filename] [watch|convert]

Parse nginx access logs and either convert them to a delimited format or
summarize the results. Reads from STDIN by default, but you can also specify an
access logfile.

The "watch" function is primarily useful for observing what traffic your routes
have been receiving, and what codes they've been returning.

The "convert" function is to facilitate piping to other analysis tools, such as
awk.

Options:
 -h             : display this message
 -f filename    : provide a filename to read for logs

Examples:
    dashlight convert -f access.log     # Prints comma-delimited list of fields
    dashlight watch -f access.log       # Summarizes the request codes
"#;

#[derive(Debug, PartialEq)]
pub enum Mode {
    CONVERT,
    WATCH,
}

#[derive(Debug)]
pub struct Config {
    pub filename: Option<String>,
    pub mode: Mode,
}

impl Config {
    // TODO: convert to OsString
    pub fn new(mut args: Vec<String>) -> Config {
        if find_flag_and_remove(&mut args, "-h").is_some() {
            eprintln!("{}", HELP_TEXT);
            process::exit(1);
        }

        match parse_args(&mut args) {
            Ok(config) => config,
            _ => {
                eprintln!("{}", USAGE_TEXT);
                process::exit(1);
            }
        }
    }
}

fn parse_args(args: &mut Vec<String>) -> Result<Config, Error> {
    // If the help flag was provided, print the help text and exit

    let filename = find_named_and_remove(args, "-f");

    let mode_str = args.get(1).ok_or(Error::ParsingError)?;
    let mode = match mode_str.as_str() {
        "watch" => Mode::WATCH,
        "convert" => Mode::CONVERT,
        _ => return Err(Error::ParsingError),
    };

    Ok(Config { filename, mode })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_read_stdin() {
        let mut args = vec!["dashlight".to_string(), "watch".into()];
        let config = parse_args(&mut args).unwrap();
        assert_eq!(config.filename, None);
    }

    #[test]
    fn one_arg_read_filename() {
        let mut args = vec![
            "dashlight".to_string(),
            "watch".into(),
            "-f".into(),
            "access.log".into(),
        ];
        let config = parse_args(&mut args).unwrap();
        assert_eq!(config.filename, Some("access.log".to_string()));
    }

    #[test]
    #[should_panic(expected = "Missing value after -f")]
    fn invalid_args_missing_filename_after_f() {
        let mut args = vec!["dashlight".to_string(), "watch".into(), "-f".into()];
        parse_args(&mut args).unwrap_err();
    }
}
