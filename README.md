# Dashlight
Dashlight is a simple, easily-installed, command-line utility for monitoring your web traffic in realtime.

## Background
_Currently under development!_

If you've ever used a web hosting service, one thing they often provide is a really nice graphical interface to monitor your web traffic and the responses that your server is sending. Dashlight recreates the essentials of that experience from your terminal by parsing your access logs.

Written to minimize unnecessary allocations, Dashlight is quite a bit faster than similar tools, and can therefore effectively monitor even extremely heavy traffic.

## Supported Formats
Right now Dashlight supports the nginx default log format, ["combined log"](https://nginx.org/en/docs/http/ngx_http_log_module.html#log_format).

## Usage
Dashlight offers two simple modes: `watch` and `convert`.

### Watch
To get a summary of your access logs, run `dashlight watch -f [FILE]`, where `[FILE]` is your access log. If no file is provided, Dashlight will read from STDIN.

It will output the sum of your request types, for instance:

```
$ dashlight watch -f tests/data/short-log
 count |   2xx |   3xx |   4xx |   5xx
 ----- + ----- + ----- + ----- + -----
     5 |     2 |     2 |     1 |     0

     route |   2xx |   3xx |   4xx |   5xx
 --------- + ----- + ----- + ----- + -----
         / |     1 |     2 |     0 |     0
 --------- + ----- + ----- + ----- + -----
      /api |     1 |     0 |     0 |     0
 --------- + ----- + ----- + ----- + -----
 /api/user |     0 |     0 |     1 |     0
```

### Convert
To convert your logs to a more easily-parsed format, use `dashlight convert`. This mode is ideal for converting passing to `awk`, `cut`, or other quick analysis tools. Note that dashlight will omit logs it was unable to parse with granularity, almost certainly because the request was purposefully malformatted by the requester.

```
$ dashlight convert -f tests/data/short-log
43.183.122.65"09/May/2022:00:00:07 +0000"GET"/"200
43.193.122.65"09/May/2022:00:00:07 +0000"GET"/"304
43.193.122.65"09/May/2022:00:00:07 +0000"GET"/"304
43.193.122.65"09/May/2022:00:00:07 +0000"POST"/api"200
43.193.122.65"09/May/2022:00:00:07 +0000"POST"/api/user"403
```

Using a single `"` as a delimiter is admittedly a bit funky, but it's one of two characters that nginx is guaranteed to escape.
