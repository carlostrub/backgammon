/// This module contains the error definition for the Backgammon game.
use std::fmt;

/// We represent all Backgammon errors as an enum
#[derive(Debug)]
pub enum Error {
    /// Game has already started
    StartedError,
    /// Game has already ended
    EndedError,
    /// Opponent is playing
    TurnError,
    /// Opponent offered dice. Need to react on this event first.
    DiceReceivedError,
    /// Doubling not permitted
    DoubleError,
}

// implement Error trait
impl std::error::Error for Error {}

// implement Display trait
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StartedError => write!(f, "Game has already started"),
            Error::EndedError => write!(f, "Game has already ended"),
            Error::TurnError => write!(f, "Opponent's turn"),
            Error::DiceReceivedError => {
                write!(
                    f,
                    "Opponent offered dice. Need to first accept or decline the doubling dice."
                )
            }
            Error::DoubleError => write!(f, "Doubling not permitted"),
        }
    }
}
