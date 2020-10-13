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
//! ## Discussions and Support
//! Any support is very welcome. Please use the following matrix room to discuss topics around this
//! crate: [#backgammon:carlostrub.ch](https://matrix.to/#/#backgammon:carlostrub.ch)
//!
//! ## Source Code Integrity
//! All commits are signed with the following GPG key (find the respective key for example in the
//!     [FreeBSD keyring](https://www.freebsd.org/doc/pgpkeyring.txt)):
//!
//! `3626 000C 0372 A78C 5DD7  B096 34EF 3FF3 3C29 811A`
//!
//! You can verify the integrity of the code by running:
//!
//! `git log --show-signature`
//!
//!

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

use std::time::SystemTime;
use uuid::Uuid;

/// Represents one single Backgammon match
#[derive(Debug)]
pub struct Match {
    id: Uuid,
    points: u32,
    rules: CurrentRules,
    games: Vec<Game>,
    time_start: SystemTime,
    time_end: SystemTime,
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

/// Implements the Backgammon rules
pub trait Rules {
    /// When offered the cube, allow to re-double but keep it.
    fn with_beaver(self) -> Self;
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    fn with_raccoon(self) -> Self;
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    fn with_murphy(self, limit: u8) -> Self;
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    fn with_jacoby(self) -> Self;
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    fn with_crawford(self) -> Self;
    /// Permits to double after Crawford game only if both players have rolled at least twice
    fn with_holland(self) -> Self;
}

/// Cube can be owned by Nobody, Player 1, or Player 2
#[derive(Debug)]
enum CubeOwner {
    Nobody,
    Player1,
    Player2,
}

/// Represents one single Backgammon game
#[derive(Debug)]
pub struct Game {
    // how many points in the game?
    points: u32,
    // if false, player 2 plays
    player1_plays: bool,
    // a board has 24 fields, #25 is the bar, #26 is the out of Player 1, #27 is the out of Player
    // 2
    board: [i8; 27],
    // this displays the n-th power of 2, e.g. 2 -> 2^2 = 4
    cube: u8,
    cube_owner: CubeOwner,
    cube_received: bool,
    // Crawford rule: if crawford game, no doubling allowed
    crawford: bool,
    // Holland rule: if <4 rolls of crawford game, no doubling allowed
    since_crawford: u8,
}

/// Implements a Backgammon game
pub mod bg_game;
/// Implements a Backgammon match
pub mod bg_match;
/// Implements all Backgammon rules
mod bg_rules;
