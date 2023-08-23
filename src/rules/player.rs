use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a player in the game.
///
/// Part of the rules of the game is that this game is for only two players, we call them Player 0
/// and Player 1. The labels are chosen arbitrarily and do not affect the game at all, however, it
/// is convenient here to use 0 and 1 as labels because we sometimes use Rust tuples which we can
/// then address the same way. There is a special case where nobody is allowed to move or act, for
/// example when a game begins or ends, thus we define this as the default.
#[derive(
    Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Player {
    /// None of the two players, e.g. at start or end of game.
    #[default]
    Nobody,
    /// Player 0
    Player0,
    /// Player 1
    Player1,
}

impl Player {
    /// Returns the other player, i.e. the player who is not the current player.
    pub fn other(&self) -> Self {
        match *self {
            Player::Nobody => Player::Nobody,
            Player::Player0 => Player::Player1,
            Player::Player1 => Player::Player0,
        }
    }
}

// Implement Display trait for Player
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Nobody => write!(f, "Nobody"),
            Player::Player0 => write!(f, "Player 0"),
            Player::Player1 => write!(f, "Player 1"),
        }
    }
}

// Test Display trait for Player
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Player::Nobody), "Nobody");
        assert_eq!(format!("{}", Player::Player0), "Player 0");
        assert_eq!(format!("{}", Player::Player1), "Player 1");
    }

    #[test]
    fn test_other() {
        assert_eq!(Player::Nobody.other(), Player::Nobody);
        assert_eq!(Player::Player0.other(), Player::Player1);
        assert_eq!(Player::Player1.other(), Player::Player0);
    }
}
