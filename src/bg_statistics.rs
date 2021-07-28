use super::Statistics;

use std::time::{Duration, SystemTime, SystemTimeError};

impl Default for Statistics {
    fn default() -> Self {
        let start = SystemTime::now();

        Statistics {
            time_start: start,
            time_end: start,
        }
    }
}

impl Statistics {
    /// Display the duration of the match in Nanoseconds
    pub(crate) fn duration(&self) -> Result<Duration, SystemTimeError> {
        self.time_end.duration_since(self.time_start)
    }

    /// Stop the clock
    pub(crate) fn stop(&mut self) {
        let stop = SystemTime::now();
        self.time_end = stop
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::{thread, time};

    #[test]
    fn rule_stop() {
        let ten_millis = time::Duration::from_millis(10);
        let s = Statistics::default();
        thread::sleep(ten_millis);
        s.stop();

        assert!(s.duration().unwrap() >= ten_millis);
    }
}
