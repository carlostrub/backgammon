/// This module contains the error definition for the Backgammon game.
use std::fmt;

/// Holds all possible errors that can occur during a Backgammon game.
#[derive(Debug)]
pub enum Error {
    /// Game has already started
    GameStarted,
    /// Game has already ended
    GameEnded,
    /// Opponent is playing, not your turn.
    PlayerTurn,
    /// Opponent offered doubling cube. Need to react on this event first.
    CubeReceived,
    /// Doubling not permitted
    DoubleNotPermitted,
    /// Invalid cube value
    CubeValueInvalid,
    /// Invalid player
    PlayerInvalid,
    /// Field blocked
    FieldBlocked,
    /// Invalid field
    FieldInvalid,
}

// implement Error trait
impl std::error::Error for Error {}

// implement Display trait
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GameStarted => write!(f, "Game has already started"),
            Error::GameEnded => write!(f, "Game has already ended"),
            Error::PlayerTurn => write!(f, "Opponent's turn"),
            Error::PlayerInvalid => write!(f, "Invalid player"),
            Error::CubeReceived => {
                write!(
                    f,
                    "Opponent offered dice. Need to first accept or decline the doubling dice."
                )
            }
            Error::CubeValueInvalid => write!(f, "Invalid cube value"),
            Error::DoubleNotPermitted => write!(f, "Doubling not permitted"),
            Error::FieldBlocked => write!(f, "Field blocked"),
            Error::FieldInvalid => write!(f, "Invalid field"),
        }
    }
}
