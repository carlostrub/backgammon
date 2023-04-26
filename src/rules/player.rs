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
