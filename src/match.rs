use crate::game::Game;
use crate::rules::{GameRules, MatchRules, Rules};

use std::fmt;

/// A Backgammon match consists of a set of rules and a vector of games
#[derive(Debug, Clone, Default)]
pub struct Match {
    /// The rules set for the match
    pub rules: Rules,
    /// The games played in the match
    pub games: Vec<Game>,
}

// implement Display trait
impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Match with rules: {:?} and Games: {:?}",
            self.rules, self.games
        )
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
    /// # use backgammon::rules::Rules;
    /// # assert_eq!(m.rules, Rules::default());
    /// ```
    pub fn new() -> Self {
        Match::default()
    }
}

/// Implements SetRules for Match
impl MatchRules for Match {
    fn with_points(mut self, points: u32) -> Self {
        self.rules.points = points;
        self
    }

    fn with_crawford(mut self) -> Self {
        self.rules.crawford = true;
        self
    }
}

/// Implements SetRules for Match
impl GameRules for Match {
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
    }

    #[test]
    fn test_new_match() {
        let m = Match::new();
        assert_eq!(m.rules, Rules::default());
        assert_eq!(m.games.len(), 0);
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
        assert!(m.rules.beaver);
        assert!(m.rules.raccoon);
        assert!(m.rules.murphy);
        assert_eq!(m.rules.murphy_limit, 3);
        assert!(m.rules.jacoby);
        assert!(m.rules.crawford);
        assert!(m.rules.holland);
    }

    #[test]
    fn test_set_points() {
        let m = Match::new().with_points(5).with_points(3);
        assert_eq!(m.rules.points, 3);
    }

    // test Display trait
    #[test]
    fn test_display() {
        let m = Match::new();
        assert_eq!(
            format!("{}", m),
            "Match with rules: Rules { points: 7, beaver: false, raccoon: false, murphy: false, murphy_limit: 0, jacoby: false, crawford: true, holland: false } and Games: []"
        );
    }
}
