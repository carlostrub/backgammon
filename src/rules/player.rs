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
