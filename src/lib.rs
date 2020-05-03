//! This is a canonical library for playeng Backgammon.
//! Some clarifications on how we play the game:
//!
//! Players: 1) Computer, 2) Opponent
//! Board:   is numerated from 1-24 beginning on the start point of the Computer. Opponent facing,
//!          this is switched and visible from the API's perspective. The bar will be used on
//!          position 25, if necessary.
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

/// Player holds informations about a Backgammon player.
#[derive(Debug, Clone, Hash)]
pub struct Player {
    /// id of the player
    pub id: Uuid,
    /// Name of the player
    pub name: String,
}

/// Match holds informations about a match
#[derive(Debug)]
pub struct Match {
    id: Uuid,
    points: u32,
    rules: Rules,
    history: Vec<Game>,
    players: (Player, Player),
    time_start: SystemTime,
    time_end: SystemTime,
}

/// Rules hold different rules for the doubling cube
#[derive(Debug)]
struct Rules {
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

/// Cube can be owned by No Player, Player 1, or Player 2
#[derive(Debug)]
enum CubeOwner {
    Nobody,
    Player1,
    Player2,
}

/// This represents a single backgammon game
#[derive(Debug)]
pub struct Game {
    points: u32,
    // this displays the n-th power of 2, e.g. 2 -> 2^2 = 4
    cube: u8,
    cube_owner: CubeOwner,
    // if false, player 2 plays
    one_plays: bool,
    // a board has 24 fields, #25 is the bar, #26 is the out of Player 1, #27 is the out of Player
    // 2
    board: [i8; 27],
    // Crawford rule: if crawford game, no doubling allowed
    crawford: bool,
    // Holland rule: if <4 rolls of crawford game, no doubling allowed
    since_crawford: u8,
}

/// This module implements a Backgammon match
pub mod bg_match;
/// This module implements all Backgammon rules
pub mod cube;
/// This module implements a Backgammon game
pub mod game;
/// This module implements Backgammon players
pub mod player;
