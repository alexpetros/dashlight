use crate::parsers::nginx;
use crate::stats::{self, StatusCodeStats};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct View {
    displayed_routes: Vec<(String, stats::StatusCodeStats)>,
    global_codes: stats::StatusCodeStats,
    codes_by_route: HashMap<String, stats::StatusCodeStats>,
}

impl View {
    pub fn new() -> View {
        View {
            global_codes: stats::StatusCodeStats::new(),
            // Only routes that we were able to parse (valid routes) go here
            codes_by_route: HashMap::new(),
            displayed_routes: vec![],
        }
    }

    pub fn update(&mut self, log: nginx::NginxCombinedLog) {
        self.global_codes.update(&log);

        let request_url = match log.request_url {
            Some(x) => x,
            None => return, // Return early on an invalid route
        };

        // Get the stats for this particular route, and update them based on the log
        let codes_for_route = self
            .codes_by_route
            .entry(String::from(request_url))
            .or_insert(stats::StatusCodeStats::new());
        codes_for_route.update(&log);

        // Update the route's position in the display based on this new information
        // It might have come into the top 10, or moved up a spot
        let position = self
            .displayed_routes
            .iter()
            .position(|item| item.0 == request_url);
        match position {
            // If the route already exists in our displayed_routes, update it
            Some(index) => self.displayed_routes[index].1 = *codes_for_route,
            // Otherwise, check whether it fits in the display
            None => {
                let route = String::from(request_url);
                if self.displayed_routes.len() < 10 {
                    // The display has a max of 10, so add it if we're under the max
                    self.displayed_routes.push((route, *codes_for_route));
                } else if codes_for_route.sum() > self.displayed_routes[9].1.sum() {
                    // Replace the lowest one (guaranteed by sort) with the current one
                    self.displayed_routes[9] = (route, *codes_for_route);
                }
                // Always sort after we replace, to guarantee that the last index holds the lowest
                self.displayed_routes
                    .sort_unstable_by_key(|a| std::cmp::Reverse(a.1.sum()))
            }
        }
    }
}

fn write_dividing_line(f: &mut fmt::Formatter, name_width: usize, num_width: usize) -> fmt::Result {
    writeln!(
        f,
        "| {0:->name_width$} + {0:->num_width$} + {0:->num_width$} + {0:->num_width$} + {0:->num_width$} |",
        ""
    )
}

fn write_stats(
    f: &mut fmt::Formatter,
    name_width: usize,
    num_width: usize,
    name: &str,
    stats: StatusCodeStats,
) -> fmt::Result {
    writeln!(
        f,
        "| {:>name_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
        name, stats.x2, stats.x3, stats.x4, stats.x5
    )
}
fn write_header(
    f: &mut fmt::Formatter,
    name_width: usize,
    num_width: usize,
    name: &str,
) -> fmt::Result {
    writeln!(
        f,
        "| {:>name_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
        name, "2xx", "3xx", "4xx", "5xx"
    )
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_width = get_string_length_of_int(self.global_codes.sum());
        let num_width = if max_width > 5 { max_width } else { 5 };
        writeln!(f, "")?;
        write_header(f, num_width, num_width, "count")?;
        write_dividing_line(f, num_width, num_width)?;
        write_stats(
            f,
            num_width,
            num_width,
            &self.global_codes.sum().to_string(),
            self.global_codes,
        )?;

        let max_width = self
            .displayed_routes
            .iter()
            .map(|x| x.0.len())
            .max()
            .unwrap();
        let name_width = if max_width > 5 { max_width } else { 5 };
        writeln!(f, "")?;
        write_header(f, name_width, num_width, "route")?;
        for (route, codes) in &self.displayed_routes {
            write_dividing_line(f, name_width, num_width)?;
            write_stats(f, name_width, num_width, route, *codes)?;
        }

        Ok(())
    }
}

fn get_string_length_of_int(num: u32) -> usize {
    // Replace with log_10 implementation at some point, bummer to have to allocate here
    // https://github.com/rust-lang/rust/issues/70887
    num.to_string().len()
}
