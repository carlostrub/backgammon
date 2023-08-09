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
        //  s.push_str(&format!("Winner: {}\n", self.winner()));
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

#[cfg(test)]
mod tests {
    use super::*;

    // new game test_default_rules
    #[test]
    fn new_test() {
        let g = Game::new();
        assert_eq!(g.rules.points, 7);
        assert!(!g.rules.beaver);
        assert!(!g.rules.raccoon);
        assert!(!g.rules.murphy);
        assert_eq!(g.rules.murphy_limit, 0);
        assert!(!g.rules.jacoby);
        assert!(g.rules.crawford);
        assert!(!g.rules.holland);
        assert_eq!(g.winner, Player::Nobody);
        assert_eq!(g.dices, (0, 0));
        assert_eq!(g.cube, 0);
        assert_eq!(g.cube_owner, Player::Nobody);
        assert_eq!(g.who_plays, Player::Nobody);
        assert_eq!(
            g.board,
            (
                [2, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2],
                (0, 0),
                (0, 0)
            )
        );
        assert!(!g.cube_received);
        assert!(!g.crawford);
        assert_eq!(g.since_crawford, 0);
    }

    #[test]
    fn roll_test() {
        let g = Game::default();
        let d = g.roll(Player::Player0).unwrap();
        assert!(d.dices.0 > 0);
        assert!(d.dices.0 < 7);
        assert!(d.dices.1 > 0);
        assert!(d.dices.1 < 7);
    }

    #[test]
    fn roll_test_fair() {
        let mut sum: u32 = 0;

        for _x in 0..100_000 {
            let g = Game::default();
            let d = g.roll(Player::Player0).unwrap();
            sum += (d.dices.0 + d.dices.1) as u32;
        }

        let average = (sum as f64) / 200_000.;
        assert!(average < 3.51);
        assert!(average > 3.49);
    }

    #[test]
    fn start_test() {
        for _x in 0..100_000 {
            let g = Game::default();
            let d = g.roll(Player::Nobody).unwrap();
            assert!(d.dices.0 != d.dices.1);
        }
    }

    // test SetRules for Game
    #[test]
    fn set_rules_test() {
        let g = Game::default()
            .with_points(5)
            .with_beaver()
            .with_raccoon()
            .with_murphy(3)
            .with_jacoby()
            .with_crawford()
            .with_holland();

        assert_eq!(g.rules.points, 5);
        assert!(g.rules.beaver);
        assert!(g.rules.raccoon);
        assert!(g.rules.murphy);
        assert_eq!(g.rules.murphy_limit, 3);
        assert!(g.rules.jacoby);
        assert!(g.rules.crawford);
        assert!(g.rules.holland);
    }

    #[test]
    fn set_points_test1() {
        let g = Game::default().with_points(5).with_points(3);

        assert_eq!(g.rules.points, 3);
    }

    #[test]
    fn set_points_test2() {
        let mut g = Game::default().with_points(5);
        g = g.with_points(3);

        assert_eq!(g.rules.points, 3);
    }
}
