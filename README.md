# Dashlight - monitor your webserver responses in realtime
Currently under development! Does not do most of the things this description promises (yet). See the Usage section for what works right now.

## Background
If you've ever used a web hosting service, one thing they often provide is a really nice graphical interface to monitor your web traffic and the responses that your server is sending. Dashlight recreates the essentials of that experience from your terminal by parsing your access logs.

Written to carefully minimize unnecessary allocations, Dashlight is quite a bit faster than similar tools, and can therefore effectively monitor even extremely heavy traffic.

## Usage
With the rust toolchain installed, simply `cargo run [FILE]`, where `[FILE]` is your access log. If no file is provided, Dashlight will read from STDIN.

It will output the sum of your request types, for instance:

```
2xx: 612660
3xx: 451344
4xx: 72137
5xx: 6490
```
