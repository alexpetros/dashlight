use crate::Error;
use std::fmt;

#[derive(Debug)]
pub struct NginxCombinedLog<'a> {
    pub remote_addr: &'a str,
    pub remote_user: &'a str,
    pub time_local: &'a str,
    pub request: &'a str,
    pub status: u32,
    pub body_bytes_sent: u32,
    pub http_referer: &'a str,
    pub http_user_agent: &'a str,
}

impl<'a> fmt::Display for NginxCombinedLog<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NginxCombinedLog {{
    remote_addr: {},
    remote_user: {},
    time_local: {},
    request: {},
    status: {},
    bytes: {},
    referer: {},
    user_agent: {}\n\
            }}",
            self.remote_addr,
            self.remote_user,
            split_at_whitespace(self.time_local).unwrap().0,
            self.request,
            self.status,
            self.body_bytes_sent,
            self.http_referer,
            split_at_whitespace(self.http_user_agent).unwrap().0,
        )
    }
}

pub fn get_log_from_logline(logline: &str) -> Result<NginxCombinedLog, Error> {
    // Break each field into its own slice of the original logline
    let (remote_addr, rest) = split_at_whitespace(&logline)?;
    let (dash, rest) = split_at_whitespace(&rest[1..])?;
    assert_char_eq(b'-', dash.as_bytes()[0])?;
    let (remote_user, rest) = split_at_whitespace(&rest[1..])?;
    assert_char_eq(b'[', rest.as_bytes()[1])?;
    let (time_local, rest) = split_at_ascii_char(b']', &rest[2..])?;
    let (request, rest) = split_at_ascii_char(b'"', &rest[3..])?;
    let (status_str, rest) = split_at_whitespace(&rest[2..])?;
    let (body_bytes_sent_str, rest) = split_at_whitespace(&rest[1..])?;
    let (http_referer, rest) = split_at_ascii_char(b'"', &rest[2..])?;
    let http_user_agent = split_at_ascii_char(b'"', &rest[3..])?.0;

    let status: u32 = status_str
        .trim()
        .parse()
        .expect("Status code is not a valid integer.");
    let body_bytes_sent: u32 = body_bytes_sent_str
        .trim()
        .parse()
        .expect("Bytes sent is not a valid integer.");

    Ok(NginxCombinedLog {
        remote_addr,
        remote_user,
        time_local,
        request,
        status,
        body_bytes_sent,
        http_referer,
        http_user_agent,
    })
}

fn assert_char_eq(expected: u8, actual: u8) -> Result<(), Error> {
    // TODO: Add message explaining what character was missing
    match expected == actual {
        true => Ok(()),
        false => Err(Error::ParsingError),
    }
}

fn split_at_ascii_char(stop_char: u8, s: &str) -> Result<(&str, &str), Error> {
    s.as_bytes()
        .iter()
        .position(|&item| item == stop_char)
        .ok_or(Error::ParsingError)
        .map(|index| (&s[..index], &s[index..]))
}

fn split_at_whitespace(s: &str) -> Result<(&str, &str), Error> {
    s.as_bytes()
        .iter()
        .position(|item| u8::is_ascii_whitespace(item))
        .ok_or(Error::ParsingError)
        .map(|index| (&s[..index], &s[index..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_logline() {
        let logline = r#"192.167.1.100 - - [09/May/2022:00:00:07 +0000] "GET / HTTP/1.1" 304 7030 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36""#;
        let log = get_log_from_logline(logline).unwrap();

        assert_eq!(log.remote_addr, "192.167.1.100");
        assert_eq!(log.remote_user, "-");
        assert_eq!(log.time_local, "09/May/2022:00:00:07 +0000");
        assert_eq!(log.request, "GET / HTTP/1.1");
        assert_eq!(log.status, 304);
        assert_eq!(log.body_bytes_sent, 7030);
        assert_eq!(log.http_referer, "-");
        assert_eq!(log.http_user_agent, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36");
    }

    #[test]
    fn parse_logline_missing_dash() {
        let logline = r#"192.167.1.100 x x [09/May/2022:00:00:07 +0000] "GET / HTTP/1.1" 304 7030 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36""#;
        assert_eq!(
            get_log_from_logline(logline).unwrap_err(),
            Error::ParsingError
        );
    }

    #[test]
    fn parse_logline_invalid_time() {
        let logline = r#"192.167.1.100 x x 09/May/2022:00:00:07 +0000 "GET / HTTP/1.1" 304 7030 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36""#;
        assert_eq!(
            get_log_from_logline(logline).unwrap_err(),
            Error::ParsingError
        );
    }
}
