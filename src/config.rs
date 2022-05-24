use std::process::exit;

const HELP_TEXT: &str = r#"Usage: dashlight [-f filename]

Reads from server access logs and provides a detailed breakdown of requests.
By default, reads from STDIN.

Options:
 -h             : display this message
 -f filename    : provide a filename to read for logs
"#;

#[derive(Debug)]
pub struct Config {
    pub filename: Option<String>,
    pub quiet: bool,
}

impl Config {
    // TODO: convert to OsString
    pub fn new(mut args: Vec<String>) -> Config {
        // If the help flag was provided, print the help text and exit
        if find_flag_and_remove(&mut args, "-h").is_some() {
            eprintln!("{}", HELP_TEXT);
            exit(0);
        }

        let filename = find_named_and_remove(&mut args, "-f");
        let quiet = find_flag_and_remove(&mut args, "-q").is_some();
        return Config { filename, quiet };
    }
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
