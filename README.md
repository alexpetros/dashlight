# Dashlight
Dashlight is a simple, easily-installed, command-line utility for monitoring your web traffic in realtime.

## Background
_Currently under development! Does not do most of the things this description promises (yet)._

If you've ever used a web hosting service, one thing they often provide is a really nice graphical interface to monitor your web traffic and the responses that your server is sending. Dashlight recreates the essentials of that experience from your terminal by parsing your access logs.

Written to carefully minimize unnecessary allocations, Dashlight is quite a bit faster than similar tools, and can therefore effectively monitor even extremely heavy traffic.

## Supported Formats
Right now Dashlight supports the nginx default log format, ["combined log"](https://nginx.org/en/docs/http/ngx_http_log_module.html#log_format).

## Usage
Simply `dashlight -f [FILE]`, where `[FILE]` is your access log. If no file is provided, Dashlight will read from STDIN.

It will output the sum of your request types, for instance:

```
$ dashlight -f access.log
Summary stats:
|   count |     2xx |     3xx |     4xx |     5xx |
| ------- + ------- + ------- + ------- + ------- |
| 1142631 |  612660 |  451344 |   72137 |    6490 |
```
