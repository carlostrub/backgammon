//! # Play a Backgammon Game
use crate::rules::Cube;
use crate::rules::Player;
use crate::rules::{Board, Move};
use crate::rules::{Dices, Roll};
use crate::rules::{GameRules, Rules};
use crate::Error;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a Backgammon game
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Game {
    /// rules of the game
    pub rules: Rules,
    /// last dice pair rolled
    pub dices: Dices,
    /// whose turn is it?
    pub who_plays: Player,
    /// board for player 0 and 1
    pub board: Board,
    /// cube value and owner
    pub cube: Cube,
    /// Crawford rule: if crawford game, no doubling allowed
    crawford: bool,
    /// Holland rule: if <4 rolls of crawford game, no doubling allowed
    since_crawford: u8,
    /// true if player has to move his checkers, i.e. after roll
    move_first: bool,
    /// if cube was offered, player has to accept first and only then can move on
    cube_received: bool,
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
        s.push_str(&format!("Board: {:?}\n", self.board.get()));
        s.push_str(&format!("Crawford game: {}\n", self.crawford));
        s.push_str(&format!("Since Crawford game: {}\n", self.since_crawford));
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

impl Roll for Game {
    fn roll(&mut self) -> Result<&mut Self, Error> {
        if self.move_first {
            return Err(Error::MoveFirst);
        }
        if self.cube_received {
            return Err(Error::CubeReceived);
        }

        self.dices = self.dices.roll();
        if self.who_plays == Player::Nobody {
            if self.dices.0 > self.dices.1 {
                self.who_plays = Player::Player0;
            } else {
                self.who_plays = Player::Player1;
            }
        }
        Ok(self)
    }
}

impl Move for Game {
    fn move_checker(&mut self, player: Player, from: usize, to: usize) -> Result<&mut Self, Error> {
        // check if player is allowed to move
        if player != self.who_plays {
            return Err(Error::NotYourTurn);
        }

        // set the checker
        self.board.set(player, to, 1)?;

        // remove old checker
        self.board.set(player, from, -1)?;

        Ok(self)
    }
}

/// Implements SetRules for Game
impl GameRules for Game {
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

    fn with_holland(mut self) -> Self {
        self.rules.holland = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() -> Result<(), Error> {
        let mut g = Game::new();
        let g = g.roll()?;
        assert!(g.dices.0 >= 1 && g.dices.0 <= 6);
        assert!(g.dices.1 >= 1 && g.dices.1 <= 6);
        Ok(())
    }

    #[test]
    fn test_move_checker() -> Result<(), Error> {
        let mut g = Game::new();
        let g = g.roll()?;
        let g = g.move_checker(g.who_plays, 23, 22)?;

        if g.who_plays == Player::Player1 {
            assert_eq!(g.board.get().board[0], -1);
            assert_eq!(g.board.get().board[1], -1);
        } else {
            assert_eq!(g.board.get().board[23], 1);
            assert_eq!(g.board.get().board[22], 1);
        }
        Ok(())
    }
}
