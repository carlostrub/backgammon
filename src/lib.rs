//! # Backgammon: The Oldest Board Game of the World
//! This crate provides a pure, canonical implementation of the game
//! [*Backgammon*](https://en.wikipedia.org/wiki/Backgammon). It allows to
//! implement fast Backgammon games in various clients.
//!
//! ## Supported Doubling Cube Rules
//! This library supports the following rules on the doubling cube:
//!
//! * Beaver
//! * Raccoon
//! * Murphy
//! * Jacoby
//! * Crawford
//! * Holland
//!
//! ## Example
//! Start a new match with rules:
//! ```
//! use backgammon::{Match,Rules};
//!
//! let mut m = Match::new().
//! with_points(13).
//! with_jacoby();
//!
//! ```
//!
//! ## Discussions and Support
//! Any support is very welcome. Please use [Bitbucket
//! Issues](https://bitbucket.org/carlostrub/backgammon/issues?status=new&status=open) to discuss
//! features or ask for help.
//!
//! ## Source Code Integrity
//! All commits are signed with the following GPG key (find the respective key for example in the
//!     [FreeBSD keyring](https://docs.freebsd.org/pgpkeys/pgpkeys.txt)):
//!
//! `59A6 2B5D B2FE B9CA 2358  4FA1 1C7A 2F39 D966 052B`
//!
//! You can verify the integrity of the code by running:
//!
//! `git log --show-signature`

#![warn(future_incompatible)]
#![deny(
    missing_docs,
    unused_variables,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)] // be tough on code quality

use std::fmt;
use std::time::SystemTime;
use uuid::Uuid;

/// Represents a Backgammon match
#[derive(Debug)]
pub struct Match {
    id: Uuid,
    points: u32,
    rules: CurrentRules,
    games: Vec<Game>,
    statistics: Statistics,
}

/// Holds the rules of the match
#[derive(Debug)]
struct CurrentRules {
    /// When offered the cube, allow to re-double but keep it.
    beaver: bool,
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    raccoon: bool,
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    murphy: bool,
    /// How often to apply automatic doubling rule. 0 means always on.
    murphy_limit: u8,
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    jacoby: bool,
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    crawford: bool,
    /// Permits to double after Crawford game only if both players have rolled at least twice
    holland: bool,
}

/// Holds various statistical information about a Match or a Game
#[derive(Debug)]
struct Statistics {
    /// start time
    time_start: SystemTime,
    /// End time
    time_end: SystemTime,
}

/// Implements the Backgammon rules
pub trait Rules {
    /// When offered the cube, allow to re-double but keep it.
    fn with_beaver(self) -> Self;
    /// Return true if beaver rule is set
    fn is_beaver(&self) -> bool;
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    fn with_raccoon(self) -> Self;
    /// Return true if Raccoon rule is set
    fn is_raccoon(&self) -> bool;
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    fn with_murphy(self, limit: u8) -> Self;
    /// Return true if Murphy rule is set
    fn is_murphy(&self) -> bool;
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    fn with_jacoby(self) -> Self;
    /// Return true if Jacoby rule is set
    fn is_jacoby(&self) -> bool;
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    fn with_crawford(self) -> Self;
    /// Return true if Crawford rule is set
    fn is_crawford(&self) -> bool;
    /// Permits to double after Crawford game only if both players have rolled at least twice
    fn with_holland(self) -> Self;
    /// Return true if Holland rule is set
    fn is_holland(&self) -> bool;
}

/// Backgammon defines certain errors
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
            Error::TurnError => write!(f, "Opponent is playing"),
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

/// This enum is used in several places, e.g. for cube ownership or for winner
#[derive(Debug, PartialEq)]
pub enum Player {
    /// none of the two players, e.g. at start
    Nobody,
    /// Player 1
    Player1,
    /// Player 2
    Player2,
}

/// Represents a Backgammon game
#[derive(Debug)]
pub struct Game {
    /// how many points in the game?
    pub points: u32,
    /// who is the winner?
    pub winner: Player,
    /// last dice pair rolled
    pub dices: (u8, u8),
    /// whose turn is it?
    pub who_plays: Player,
    /// a board has 24 fields, the second tuple is the bar for Player 1 and 2, the third tuple is
    /// the off for Player 1 and 2
    pub board: ([i8; 24], (u8, u8), (u8, u8)),
    /// cube displays the n-th power of 2, e.g. 2 -> 2^2 = 4
    pub cube: u8,
    /// who holds the cube
    pub cube_owner: Player,
    /// was cube offered to the one who plays?
    pub cube_received: bool,
    // Crawford rule: if crawford game, no doubling allowed
    crawford: bool,
    // Holland rule: if <4 rolls of crawford game, no doubling allowed
    since_crawford: u8,
    // Gather statistical information
    statistics: Statistics,
}

/// Implements a Backgammon game
mod bg_game;
/// Implements a Backgammon match
mod bg_match;
/// Implements all Backgammon rules
mod bg_rules;
/// Implements certain Backgammon statistics
mod bg_statistics;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn debug_match() {
        let m = Match::new().with_jacoby();

        assert_eq!(
            format!("The match rules are: {:?}", m.rules),
            "The match rules are: CurrentRules { beaver: false, raccoon: false, murphy: false, murphy_limit: 0, jacoby: true, crawford: true, holland: false }"
        );
    }

    #[test]
    fn debug_current_rules() {
        let r = CurrentRules::default().with_jacoby();

        assert_eq!(
            format!("The match rules are: {:?}", r),
            "The match rules are: CurrentRules { beaver: false, raccoon: false, murphy: false, murphy_limit: 0, jacoby: true, crawford: true, holland: false }"
        );
    }

    #[test]
    fn debug_cubeowner() {
        let o = Player::Nobody;

        assert_eq!(
            format!("The cube is owned by: {:?}", o),
            "The cube is owned by: Nobody"
        );
    }
    #[test]
    fn debug_game() {
        let g = Game::default();

        assert_eq!(
            format!("The game is: {:?}", g),
            "The game is: Game { points: 0, dices: (0, 0), who_plays: Nobody, board: [2, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2, 0, 0, 0], cube: 0, cube_owner: Nobody, cube_received: false, crawford: false, since_crawford: 0 }"
        );
    }
}
