use std::fmt;

#[derive(Debug)]
pub struct NginxCombinedLog<'a> {
    remote_addr: &'a str,
    remote_user: &'a str,
    time_local: &'a str,
    request: &'a str,
    status: u32,
    body_bytes_sent: u32,
    http_referer: &'a str,
    http_user_agent: &'a str,
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
            slice_to_whitespace(0, self.time_local).1,
            self.request,
            self.status,
            self.body_bytes_sent,
            self.http_referer,
            slice_to_whitespace(0, self.http_user_agent).1
        )
    }
}

pub fn get_log_from_logline(logline: &str) -> NginxCombinedLog {
    let (i, remote_addr) = slice_to_whitespace(0, &logline);
    let (i, _dash) = slice_to_whitespace(i + 1, &logline);
    let (i, remote_user) = slice_to_whitespace(i + 1, &logline);
    let (i, time_local) = slice_to_ascii_char(i + 2, b']', &logline);
    let (i, request) = slice_to_ascii_char(i + 3, b'"', &logline);
    let (i, status_str) = slice_to_whitespace(i + 2, &logline);
    let (i, body_bytes_sent_str) = slice_to_whitespace(i + 1, &logline);
    let (i, http_referer) = slice_to_ascii_char(i + 2, b'"', &logline);
    let (_i, http_user_agent) = slice_to_ascii_char(i + 3, b'"', &logline);

    let status: u32 = status_str
        .trim()
        .parse()
        .expect("Status code is not a valid integer.");
    let body_bytes_sent: u32 = body_bytes_sent_str
        .trim()
        .parse()
        .expect("Bytes sent is not a valid integer.");

    NginxCombinedLog {
        remote_addr,
        remote_user,
        time_local,
        request,
        status,
        body_bytes_sent,
        http_referer,
        http_user_agent,
    }
}

fn slice_to_ascii_char(start: usize, stop_char: u8, s: &str) -> (usize, &str) {
    let remaining_str = &s[start..];
    for (i, &item) in remaining_str.as_bytes().iter().enumerate() {
        if item == stop_char {
            return (start + i, &remaining_str[..i]);
        }
    }
    (0, &s[..])
}

fn slice_to_whitespace(start: usize, s: &str) -> (usize, &str) {
    let remaining_str = &s[start..];
    for (i, &item) in remaining_str.as_bytes().iter().enumerate() {
        if u8::is_ascii_whitespace(&item) {
            return (start + i, &remaining_str[..i]);
        }
    }
    (0, &s[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_logline() {
        let logline = r#"192.167.1.100 - - [09/May/2022:00:00:07 +0000] "GET / HTTP/1.1" 304 7030 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36""#;
        let log = get_log_from_logline(logline);

        assert_eq!(log.remote_addr, "192.167.1.100");
        assert_eq!(log.remote_user, "-");
        assert_eq!(log.time_local, "09/May/2022:00:00:07 +0000");
        assert_eq!(log.request, "GET / HTTP/1.1");
        assert_eq!(log.status, 304);
        assert_eq!(log.body_bytes_sent, 7030);
        assert_eq!(log.http_referer, "-");
        assert_eq!(log.http_user_agent, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36");
    }
}
