use crate::parsers::nginx;

#[derive(Debug)]
pub struct StatusCodeStats {
    // Items are so named because 2xx, 3xx, etc. would be illegal variable names
    pub x2: u32,
    pub x3: u32,
    pub x4: u32,
    pub x5: u32,
}

impl StatusCodeStats {
    pub fn new() -> StatusCodeStats {
        StatusCodeStats {
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
        }
    }

    // Given a logline, incremement the counter of the appropriate error code
    pub fn update(&mut self, log: nginx::NginxCombinedLog) {
        match log.status {
            200..=299 => self.x2 += 1,
            300..=399 => self.x3 += 1,
            400..=499 => self.x4 += 1,
            500..=599 => self.x5 += 1,
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_inits_to_zero() {
        let codes = StatusCodeStats::new();
        assert_eq!(codes.x2, 0);
        assert_eq!(codes.x3, 0);
        assert_eq!(codes.x4, 0);
        assert_eq!(codes.x5, 0);
    }

    #[test]
    fn update_with_200_increments_2xx() {
        let mut codes = StatusCodeStats::new();
        let mut log = nginx::NginxCombinedLog::new_blank();
        log.status = 200;
        codes.update(log);

        assert_eq!(codes.x2, 1);
        assert_eq!(codes.x3, 0);
        assert_eq!(codes.x4, 0);
        assert_eq!(codes.x5, 0);
    }

    #[test]
    fn update_with_304_increments_3xx() {
        let mut codes = StatusCodeStats::new();
        let mut log = nginx::NginxCombinedLog::new_blank();
        log.status = 304;
        codes.update(log);

        assert_eq!(codes.x2, 0);
        assert_eq!(codes.x3, 1);
        assert_eq!(codes.x4, 0);
        assert_eq!(codes.x5, 0);
    }

    #[test]
    fn update_with_404_increments_4xx() {
        let mut codes = StatusCodeStats::new();
        let mut log = nginx::NginxCombinedLog::new_blank();
        log.status = 404;
        codes.update(log);

        assert_eq!(codes.x2, 0);
        assert_eq!(codes.x3, 0);
        assert_eq!(codes.x4, 1);
        assert_eq!(codes.x5, 0);
    }

    #[test]
    fn update_with_500_increments_5xx() {
        let mut codes = StatusCodeStats::new();
        let mut log = nginx::NginxCombinedLog::new_blank();
        log.status = 500;
        codes.update(log);

        assert_eq!(codes.x2, 0);
        assert_eq!(codes.x3, 0);
        assert_eq!(codes.x4, 0);
        assert_eq!(codes.x5, 1);
    }
}
