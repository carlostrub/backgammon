use std::time::SystemTime;

use super::CurrentRules;
use super::Match;
use super::Rules;

use uuid::Uuid;

impl Default for Match {
    fn default() -> Self {
        let id = Uuid::new_v4();
        let time_start = SystemTime::now();

        Match {
            id,
            points: 0,
            rules: CurrentRules::default(),
            games: Vec::new(),
            time_start,
            time_end: time_start,
        }
    }
}

/// Implements methods for the Match struct
impl Match {
    /// Create a new Match
    pub fn new() -> Self {
        Match::default()
    }
}

impl Rules for Match {
    fn with_beaver(self) -> Self {
        Match {
            rules: self.rules.with_beaver(),
            ..self
        }
    }

    fn with_raccoon(self) -> Self {
        Match {
            rules: self.rules.with_raccoon(),
            ..self
        }
    }

    fn with_murphy(self, limit: u8) -> Self {
        Match {
            rules: self.rules.with_murphy(limit),
            ..self
        }
    }

    fn with_jacoby(self) -> Self {
        Match {
            rules: self.rules.with_jacoby(),
            ..self
        }
    }

    fn with_crawford(self) -> Self {
        Match {
            rules: self.rules.with_crawford(),
            ..self
        }
    }

    fn with_holland(self) -> Self {
        Match {
            rules: self.rules.with_holland(),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {}
