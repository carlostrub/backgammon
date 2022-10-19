/// Backgammon requires certain game specific errors
use std::fmt;

/// Represents a Backgammon error
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
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StartedError => write!(f, "Game has already started"),
            Error::EndedError => write!(f, "Game has already ended"),
            Error::TurnError => write!(f, "Other player's turn"),
            Error::DiceReceivedError => {
                write!(
                    f,
                    "Opponent offered dice. Need to react on this event first."
                )
            }
            Error::DoubleError => write!(f, "Doubling not permitted"),
        }
    }
}
