use std::fmt;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct NginxCombinedLog<'a> {
    remote_addr: &'a str,
    remote_user: &'a str,
    time_local: &'a str,
    request: &'a str,
    status: &'a str,
    body_bytes_sent: &'a str,
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

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let logline = line.unwrap();

        let (i, ip) = slice_to_whitespace(0, &logline);
        let (i, _dash) = slice_to_whitespace(i + 1, &logline);
        let (i, user) = slice_to_whitespace(i + 1, &logline);
        let (i, time_local) = slice_to_ascii_char(i + 2, b']', &logline);
        let (i, request) = slice_to_ascii_char(i + 3, b'"', &logline);
        let (i, status) = slice_to_whitespace(i + 2, &logline);
        let (i, body_bytes_sent) = slice_to_whitespace(i + 1, &logline);
        let (i, http_referer) = slice_to_ascii_char(i + 2, b'"', &logline);
        let (_i, http_user_agent) = slice_to_ascii_char(i + 3, b'"', &logline);

        let log_entry = NginxCombinedLog {
            remote_addr: ip,
            remote_user: user,
            time_local: time_local,
            request: request,
            status: status,
            body_bytes_sent: body_bytes_sent,
            http_referer: http_referer,
            http_user_agent: http_user_agent,
        };

        println!("{:?}", log_entry);
    }
}
