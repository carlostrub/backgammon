//! # Backgammon Game
//! Start a game by calling:
//! ```
//! use backgammon::game::Game;
//! use backgammon::rules::player::Player;
//!
//! let mut g = Game::new().roll(Player::Nobody).unwrap();
//!
//! println!("{}", g);
//! ```
use crate::error::Error;
use crate::rules::board::Board;
use crate::rules::cube::Cube;
use crate::rules::player::Player;
use crate::rules::{Rules, SetRules};

use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a Backgammon game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// rules of the game
    pub rules: Rules,
    /// last dice pair rolled
    pub dices: (u8, u8),
    /// whose turn is it?
    pub who_plays: Player,
    /// board for player 0 and 1
    board: Board,
    /// cube value and owner
    cube: Cube,
    /// was cube offered to the one who plays?
    pub cube_received: bool,
    /// Crawford rule: if crawford game, no doubling allowed
    pub crawford: bool,
    /// Holland rule: if <4 rolls of crawford game, no doubling allowed
    pub since_crawford: u8,
}

// Backgammon uses 15 checkers per side
//const CHECKERS: u8 = 15;
impl Default for Game {
    fn default() -> Self {
        Game {
            rules: Rules::default(),
            dices: (0, 0),
            cube: Cube::default(),
            who_plays: Player::Nobody,
            board: Board::default(),
            cube_received: false,
            crawford: false,
            since_crawford: 0,
        }
    }
}

// implement Display trait
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("Rules: {}\n", self.rules));
        s.push_str(&format!("Dices: {:?}\n", self.dices));
        s.push_str(&format!("Cube: {}\n", self.cube.value()));
        s.push_str(&format!("Cube owner: {}\n", self.cube.owner()));
        s.push_str(&format!("Who plays: {}\n", self.who_plays));
        s.push_str(&format!("Board: {:?}\n", self.board));
        s.push_str(&format!("Crawford: {}\n", self.crawford));
        s.push_str(&format!("Since Crawford: {}\n", self.since_crawford));
        write!(f, "{}", s)
    }
}

impl Game {
    /// Create a new default game
    pub fn new() -> Self {
        Game::default()
    }

    // Winner of the game
    //pub fn winner(&self) -> Player {}

    /// Roll the dices which generates two random numbers between 1 and 6, replicating a perfect
    /// dice. We use the operating systems random number generator.
    pub fn roll(mut self, p: Player) -> Result<Self, Error> {
        let between = Uniform::new_inclusive(1, 6);
        let mut rng = rand::thread_rng();

        // Only roll if it is the turn of the player or if nobody has the turn (which means the
        // game starts)
        match self.who_plays == p || self.who_plays == Player::Nobody {
            false => Err(Error::Turn),
            true => match self.cube_received {
                true => Err(Error::CubeReceived),
                false => {
                    self.dices = (between.sample(&mut rng), between.sample(&mut rng));
                    if self.who_plays == Player::Nobody && self.dices.0 != self.dices.1 {
                        if self.dices.0 > self.dices.1 {
                            self.who_plays = Player::Player0;
                        } else {
                            self.who_plays = Player::Player1;
                        }
                    }
                    Ok(self)
                }
            },
        }
    }

    //    fn calculate_free_positions(&mut self) {
    //        // set free positions of computer to zero
    //        self.free_positions_computer = 0;
    //        self.free_positions_opponent = 0;
    //
    //        // check bar first
    //        if self.accounting.board[24] > 0 {
    //            for i in 0..5 {
    //                if self.accounting.board[i] > -2 {
    //                    self.free_positions_computer |= 2u32.pow(i as u32);
    //                }
    //            }
    //        } else {
    //            for i in 0..23 {
    //                if self.accounting.board[i] > -2 {
    //                    self.free_positions_computer |= 2u32.pow(i as u32);
    //                }
    //            }
    //
    //            if self.accounting.board[25] > 0 {
    //                // set free positions of computer to zer
    //                self.free_positions_opponent = 0;
    //                for i in 18..23 {
    //                    if self.accounting.board[i] > -1 {
    //                        self.free_positions_computer |= 2u32.pow(i as u32);
    //                    }
    //                }
    //            }
    //        }
    //    }
}

/// Implements SetRules for Game
impl SetRules for Game {
    fn with_points(mut self, points: u32) -> Self {
        self.rules.points = points;
        self
    }

    fn with_beaver(mut self) -> Self {
        self.rules.beaver = true;
        self
    }

    fn with_raccoon(mut self) -> Self {
        self.rules.raccoon = true;
        self
    }

    fn with_murphy(mut self, limit: u8) -> Self {
        self.rules.murphy = true;
        self.rules.murphy_limit = limit;
        self
    }

    fn with_jacoby(mut self) -> Self {
        self.rules.jacoby = true;
        self
    }

    fn with_crawford(mut self) -> Self {
        self.rules.crawford = true;
        self
    }

    fn with_holland(mut self) -> Self {
        self.rules.holland = true;
        self
    }
}

// Unit tests
