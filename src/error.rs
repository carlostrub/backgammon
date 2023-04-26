/// This module contains the error definition for the Backgammon game.
use std::fmt;

/// We represent all Backgammon errors as an enum
#[derive(Debug)]
pub enum Error {
    /// Game has already started
    Started,
    /// Game has already ended
    Ended,
    /// Opponent is playing
    Turn,
    /// Opponent offered dice. Need to react on this event first.
    DiceReceived,
    /// Doubling not permitted
    Double,
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
            Error::DiceReceived => {
                write!(
                    f,
                    "Opponent offered dice. Need to first accept or decline the doubling dice."
                )
            }
            Error::Double => write!(f, "Doubling not permitted"),
        }
    }
}
