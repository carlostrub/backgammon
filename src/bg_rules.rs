/// This module contains all the rules for the game of Backgammon
use std::fmt;

/// Part of the rules of the game is that this game is for only two players. In some cases, nobody
/// is allowed to move, thus we define this as the default
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash, Default)]
pub enum Player {
    /// none of the two players, e.g. at start
    #[default]
    Nobody,
    /// Player 1
    Player1,
    /// Player 2
    Player2,
}

// Implement Display trait for Player
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Nobody => write!(f, "Nobody"),
            Player::Player1 => write!(f, "Player 1"),
            Player::Player2 => write!(f, "Player 2"),
        }
    }
}

/// Holds the rules of the match
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Rules {
    /// The amount of points to reach for declaring a winner, default is 7.
    points: u32,
    /// When offered the cube, allow to re-double but keep it, default is false.
    beaver: bool,
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    /// Default is false
    raccoon: bool,
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board. Default is false.
    murphy: bool,
    /// How often to apply automatic doubling rule. 0 means always on. Default is 0.
    murphy_limit: u8,
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered. Default is false.
    jacoby: bool,
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game. Default is true.
    crawford: bool,
    /// Permits to double after Crawford game only if both players have rolled at least twice.
    /// Default is false.
    holland: bool,
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

trait DefineRules {
    fn set_points(self, p: u32) -> Self {
        Rules { points: p, ..self }
    }
    fn with_beaver(self) -> Self {
        Rules {
            beaver: true,
            ..self
        }
    }
    fn is_beaver(&self) -> bool {
        self.beaver
    }

    fn with_raccoon(self) -> Self {
        Rules {
            beaver: true,
            raccoon: true,
            ..self
        }
    }
    fn is_raccoon(&self) -> bool {
        self.raccoon
    }

    fn with_murphy(self, limit: u8) -> Self {
        Rules {
            murphy: true,
            murphy_limit: limit,
            ..self
        }
    }
    fn is_murphy(&self) -> bool {
        self.murphy
    }
    fn murphy_limit(&self) -> u8 {
        self.murphy_limit
    }

    fn with_jacoby(self) -> Self {
        Rules {
            jacoby: true,
            ..self
        }
    }
    fn is_jacoby(&self) -> bool {
        self.jacoby
    }

    fn with_crawford(self) -> Self {
        Rules {
            crawford: true,
            ..self
        }
    }
    fn is_crawford(&self) -> bool {
        self.crawford
    }

    fn with_holland(self) -> Self {
        Rules {
            crawford: true,
            holland: true,
            ..self
        }
    }
    fn is_holland(&self) -> bool {
        self.holland
    }
}
