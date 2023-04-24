use crate::bg_game::Game;
use crate::bg_rules::{Rules, SetRules};

use std::fmt;
use uuid::Uuid;

/// A Backgammon match consists of an Id (to be used in applications calling this library), a set
/// of rules and a vector of games
#[derive(Debug, Clone)]
pub struct Match {
    id: Uuid,
    rules: Rules,
    games: Vec<Game>,
}

impl Default for Match {
    fn default() -> Self {
        Match {
            id: Uuid::new_v4(),
            rules: Rules::default(),
            games: Vec::new(),
        }
    }
}

// implement Display trait
impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Match: {}, Games: {:?}", self.id, self.games)
    }
}

/// Implements methods for the Match struct
impl Match {
    /// Start a new match:
    /// ```
    /// use backgammon::Match;
    ///
    /// let m = Match::new();
    ///
    /// # use backgammon::Rules;
    /// # assert_eq!(m.rules, Rules::default());
    /// ```
    pub fn new() -> Self {
        Match::default()
    }
}

/// Implements SetRules for Match
impl SetRules for Match {
    fn with_points(mut self, points: u32) -> Self {
        self.rules.points = points;
        self
    }

    fn with_beaver(mut self) -> Self {
        self.rules.beaver = true;
        self
    }

    fn with_raccoon(mut self) -> Self {
        self.rules.raccoon = true;
        self
    }

    fn with_murphy(mut self, limit: u8) -> Self {
        self.rules.murphy = true;
        self.rules.murphy_limit = limit;
        self
    }

    fn with_jacoby(mut self) -> Self {
        self.rules.jacoby = true;
        self
    }

    fn with_crawford(mut self) -> Self {
        self.rules.crawford = true;
        self
    }

    fn with_holland(mut self) -> Self {
        self.rules.holland = true;
        self
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_match() {
        let m = Match::default();
        assert_eq!(m.rules, Rules::default());
        assert_eq!(m.games.len(), 0);
        assert_eq!(m.id.get_version_num(), 4);
    }

    #[test]
    fn test_new_match() {
        let m = Match::new();
        assert_eq!(m.rules, Rules::default());
        assert_eq!(m.games.len(), 0);
        assert_eq!(m.id.get_version_num(), 4);
    }

    #[test]
    fn test_set_rules() {
        let m = Match::new()
            .with_points(5)
            .with_beaver()
            .with_raccoon()
            .with_murphy(3)
            .with_jacoby()
            .with_crawford()
            .with_holland();
        assert_eq!(m.rules.points, 5);
        assert_eq!(m.rules.beaver, true);
        assert_eq!(m.rules.raccoon, true);
        assert_eq!(m.rules.murphy, true);
        assert_eq!(m.rules.murphy_limit, 3);
        assert_eq!(m.rules.jacoby, true);
        assert_eq!(m.rules.crawford, true);
        assert_eq!(m.rules.holland, true);
    }
}
