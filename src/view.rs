use crate::parsers::nginx;
use crate::stats::{self, StatusCodeStats};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct View {
    displayed_routes: Vec<(String, stats::StatusCodeStats)>,
    global_codes: stats::StatusCodeStats,
    route_codes: HashMap<String, stats::StatusCodeStats>,
}

impl View {
    pub fn new() -> View {
        View {
            global_codes: stats::StatusCodeStats::new(),
            route_codes: HashMap::new(),
            displayed_routes: vec![],
        }
    }

    pub fn update(&mut self, log: nginx::NginxCombinedLog) {
        self.global_codes.update(&log);
        let route_codes = self
            .route_codes
            .entry(String::from(log.request))
            .or_insert(stats::StatusCodeStats::new());
        route_codes.update(&log);

        let position = self
            .displayed_routes
            .iter()
            .position(|item| item.0 == log.request);
        match position {
            // If the route already exists in our displayed_routes, update it
            Some(index) => self.displayed_routes[index].1 = *route_codes,
            // Otherwise, check whether it fits in the display
            None => {
                let route = String::from(log.request);
                if self.displayed_routes.len() < 10 {
                    // The display has a max of 10, so add it if we're under the max
                    self.displayed_routes.push((route, *route_codes));
                } else if route_codes.sum() > self.displayed_routes[9].1.sum() {
                    // Replace the lowest one (guaranteed by sort) with the current one
                    self.displayed_routes[9] = (route, *route_codes);
                }
                // Always sort after we replace, to guarantee that the last index holds the lowest
                self.displayed_routes
                    .sort_unstable_by_key(|a| std::cmp::Reverse(a.1.sum()))
            }
        }
    }
}


impl fmt::Display for View {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_width = get_string_length_of_int(self.global_codes.sum());
        let width = if max_width > 5 { max_width } else { 5 };
        let StatusCodeStats { x2, x3, x4, x5 } = self.global_codes;
        writeln!(f, "Summary stats:")?;
        writeln!(
            f,"| {:>width$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |",
            "count", "2xx", "3xx", "4xx", "5xx"
        )?;

        writeln!(
            f,
            "| {0:->width$} + {0:->width$} + {0:->width$} + {0:->width$} + {0:->width$} |",
            ""
        )?;


        writeln!(
            f,
            "| {:>width$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |\n",
            self.global_codes.sum(), x2, x3, x4, x5
        )?;


        for (route, codes) in &self.displayed_routes {
            writeln!(f, "{} {}", route, codes)?;
        }

        Ok(())
    }
}

fn get_string_length_of_int(num: u32) -> usize {
    // Replace with log_10 implementation at some point, bummer to have to allocate here
    // https://github.com/rust-lang/rust/issues/70887
    num.to_string().len()
}
