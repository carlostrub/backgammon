/// This module contains the error definition for the Backgammon game.
use std::fmt;

/// Holds all possible errors that can occur during a Backgammon game.
#[derive(Debug)]
pub enum Error {
    /// Game has already started
    Started,
    /// Game has already ended
    Ended,
    /// Opponent is playing, not your turn.
    Turn,
    /// Opponent offered doubling cube. Need to react on this event first.
    CubeReceived,
    /// Doubling not permitted
    Double,
    /// Invalid cube value
    InvalidCubeValue,
    /// Invalid player
    InvalidPlayer,
}

// implement Error trait
impl std::error::Error for Error {}

// implement Display trait
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Started => write!(f, "Game has already started"),
            Error::Ended => write!(f, "Game has already ended"),
            Error::Turn => write!(f, "Opponent's turn"),
            Error::CubeReceived => {
                write!(
                    f,
                    "Opponent offered dice. Need to first accept or decline the doubling dice."
                )
            }
            Error::Double => write!(f, "Doubling not permitted"),
            Error::InvalidCubeValue => write!(f, "Invalid cube value"),
            Error::InvalidPlayer => write!(f, "Invalid player"),
        }
    }
}
