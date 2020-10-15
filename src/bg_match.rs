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
    /// Start a new match:
    /// ```
    /// use backgammon::{Match,Rules};
    ///
    /// let m = Match::new().
    /// with_raccoon().
    /// with_jacoby();
    ///
    /// # assert!(m.is_beaver(),true);
    /// ```
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
    fn is_beaver(self) -> bool {
        self.rules.is_beaver()
    }

    fn with_raccoon(self) -> Self {
        Match {
            rules: self.rules.with_raccoon(),
            ..self
        }
    }
    fn is_raccoon(self) -> bool {
        self.rules.is_raccoon()
    }

    fn with_murphy(self, limit: u8) -> Self {
        Match {
            rules: self.rules.with_murphy(limit),
            ..self
        }
    }
    fn is_murphy(self) -> bool {
        self.rules.is_murphy()
    }

    fn with_jacoby(self) -> Self {
        Match {
            rules: self.rules.with_jacoby(),
            ..self
        }
    }
    fn is_jacoby(self) -> bool {
        self.rules.is_jacoby()
    }

    fn with_crawford(self) -> Self {
        Match {
            rules: self.rules.with_crawford(),
            ..self
        }
    }
    fn is_crawford(self) -> bool {
        self.rules.is_crawford()
    }

    fn with_holland(self) -> Self {
        Match {
            rules: self.rules.with_holland(),
            ..self
        }
    }
    fn is_holland(self) -> bool {
        self.rules.is_holland()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rule_test_murphy() {
        let m = Match::new().with_murphy(3);

        assert!(m.is_murphy(), true);
    }

    #[test]
    fn rule_test_jacoby() {
        let m = Match::new().with_jacoby();

        assert!(m.is_jacoby(), true);
    }

    #[test]
    fn rule_test_crawford() {
        let m = Match::new().with_crawford();

        assert!(m.is_crawford(), true);
    }

    #[test]
    fn rule_test_holland() {
        let m = Match::new().with_holland();

        assert!(m.is_holland(), true);
    }
}
