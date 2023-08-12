/// Implements the board
mod board;
pub use board::Board;
pub use board::BoardDisplay;
/// Implements the double dice or cube
mod cube;
pub use cube::Cube;
/// Implements the players
mod player;
pub use player::Player;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Holds all the rule settings
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Rules {
    /// The amount of points to reach for declaring a winner, default is 7.
    pub points: u32,
    /// When offered the cube, allow to re-double but keep it, default is false.
    pub beaver: bool,
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    /// Default is false
    pub raccoon: bool,
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board. Default is false.
    pub murphy: bool,
    /// How often to apply automatic doubling rule. 0 means always on. Default is 0.
    pub murphy_limit: u8,
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered. Default is false.
    pub jacoby: bool,
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game. Default is true.
    pub crawford: bool,
    /// Permits to double after Crawford game only if both players have rolled at least twice.
    /// Default is false.
    pub holland: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            points: 7,
            beaver: false,
            raccoon: false,
            murphy: false,
            murphy_limit: 0,
            jacoby: false,
            crawford: true,
            holland: false,
        }
    }
}

// implement Display trait
impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Points: {}, Beaver: {}, Raccoon: {}, Murphy: {}, Murphy Limit: {}, Jacoby: {}, Crawford: {}, Holland: {}",
            self.points, self.beaver, self.raccoon, self.murphy, self.murphy_limit, self.jacoby, self.crawford, self.holland
        )
    }
}

/// Allows to modify the rules
pub trait SetRules {
    /// Set the amount of points to reach for declaring a winner
    fn with_points(self, points: u32) -> Self;
    /// When offered the cube, allow to re-double but keep it
    fn with_beaver(self) -> Self;
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube
    fn with_raccoon(self) -> Self;
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    fn with_murphy(self, limit: u8) -> Self;
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered
    fn with_jacoby(self) -> Self;
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game
    fn with_crawford(self) -> Self;
    /// Permits to double after Crawford game only if both players have rolled at least twice
    fn with_holland(self) -> Self;
}

/// Implements SetRules for Rules
impl SetRules for Rules {
    fn with_points(mut self, points: u32) -> Self {
        self.points = points;
        self
    }

    fn with_beaver(mut self) -> Self {
        self.beaver = true;
        self
    }

    fn with_raccoon(mut self) -> Self {
        self.raccoon = true;
        self
    }

    fn with_murphy(mut self, limit: u8) -> Self {
        self.murphy = true;
        self.murphy_limit = limit;
        self
    }

    fn with_jacoby(mut self) -> Self {
        self.jacoby = true;
        self
    }

    fn with_crawford(mut self) -> Self {
        self.crawford = true;
        self
    }

    fn with_holland(mut self) -> Self {
        self.holland = true;
        self
    }
}

/// Test if default rule is created correctly and if the rules can be modified
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rules() {
        let rules = Rules::default();
        assert_eq!(rules.points, 7);
        assert!(!rules.beaver);
        assert!(!rules.raccoon);
        assert!(!rules.murphy);
        assert_eq!(rules.murphy_limit, 0);
        assert!(!rules.jacoby);
        assert!(rules.crawford);
        assert!(!rules.holland);
    }

    #[test]
    fn test_set_rules() {
        let rules = Rules::default()
            .with_points(5)
            .with_beaver()
            .with_raccoon()
            .with_murphy(3)
            .with_jacoby()
            .with_crawford()
            .with_holland();
        assert_eq!(rules.points, 5);
        assert!(rules.beaver);
        assert!(rules.raccoon);
        assert!(rules.murphy);
        assert_eq!(rules.murphy_limit, 3);
        assert!(rules.jacoby);
        assert!(rules.crawford);
        assert!(rules.holland);
    }

    #[test]
    fn test_with_holland() {
        let rules = Rules::default().with_holland();
        assert!(rules.crawford);
    }

    #[test]
    fn test_with_raccoon() {
        let rules = Rules::default().with_raccoon();
        assert!(rules.raccoon);
    }
}
