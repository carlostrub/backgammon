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
    /// true if player needs to roll first
    roll_first: bool,
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
}

impl Roll for Game {
    fn roll(&mut self) -> Result<&mut Self, Error> {
        if !self.dices.consumed.0
            && !self.dices.consumed.1
            && !self.dices.consumed.2
            && !self.dices.consumed.3
        {
            return Err(Error::MoveFirst);
        }
        if self.cube_received {
            return Err(Error::CubeReceived);
        }

        self.dices = self.dices.roll();
        if self.who_plays == Player::Nobody {
            let diff = self.dices.values.0 - self.dices.values.1;
            match diff {
                0 => {
                    self.who_plays = Player::Nobody;
                }
                _ if diff > 0 => {
                    self.who_plays = Player::Player0;
                }
                _ => {
                    self.who_plays = Player::Player1;
                }
            }
        }
        Ok(self)
    }
}

impl Move for Game {
    fn move_checker(&mut self, player: Player, dice: u8, from: usize) -> Result<&mut Self, Error> {
        // check if move is permitted
        let _ = self.move_permitted(player, dice)?;

        // check if player has to move checker from bar first
        if ((player == Player::Player0) && (self.board.get().bar.0 > 0))
            || ((player == Player::Player1) && (self.board.get().bar.1 > 0))
        {
            return Err(Error::MoveInvalidBar);
        }

        // check if the dice value has been consumed
        if (dice == self.dices.values.0 && self.dices.consumed.0)
            || (dice == self.dices.values.1 && self.dices.consumed.1)
            || (dice == self.dices.values.1 && self.dices.consumed.2)
            || (dice == self.dices.values.1 && self.dices.consumed.3)
        {
            return Err(Error::MoveInvalid);
        }

        // remove checker from old position
        self.board.set(player, from, -1)?;

        // move checker to new position, in case it is reaching the off position, set it off
        let new_position = from as i8 - dice as i8;
        if new_position < 0 {
            self.board.set_off(player, 1)?;
        } else {
            self.board.set(player, new_position as usize, 1)?;
        }

        // set dice value to consumed
        if dice == self.dices.values.0 && !self.dices.consumed.0 {
            self.dices.consumed.0 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.1 {
            self.dices.consumed.1 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.2 {
            self.dices.consumed.2 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.3 {
            self.dices.consumed.3 = true;
        }

        // switch to other player if all dices have been consumed
        if self.dices.consumed.0
            && self.dices.consumed.1
            && self.dices.consumed.2
            && self.dices.consumed.3
        {
            self.who_plays = self.who_plays.other();
            self.roll_first = true;
        }

        Ok(self)
    }

    fn move_checker_from_bar(&mut self, player: Player, dice: u8) -> Result<&mut Self, Error> {
        // check if move is permitted
        let _ = self.move_permitted(player, dice)?;

        // check if the dice value has been consumed
        if (dice == self.dices.values.0 && self.dices.consumed.0)
            || (dice == self.dices.values.1 && self.dices.consumed.1)
            || (dice == self.dices.values.1 && self.dices.consumed.2)
            || (dice == self.dices.values.1 && self.dices.consumed.3)
        {
            return Err(Error::MoveInvalid);
        }

        // set the checker from bar
        self.board.set_bar(player, -1)?;
        self.board.set(player, 24 - dice as usize, 1)?;

        // set dice value to consumed
        if dice == self.dices.values.0 && !self.dices.consumed.0 {
            self.dices.consumed.0 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.1 {
            self.dices.consumed.1 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.2 {
            self.dices.consumed.2 = true;
        } else if dice == self.dices.values.1 && !self.dices.consumed.3 {
            self.dices.consumed.3 = true;
        }

        // switch to other player if all dices have been consumed
        if self.dices.consumed.0
            && self.dices.consumed.1
            && self.dices.consumed.2
            && self.dices.consumed.3
        {
            self.who_plays = self.who_plays.other();
            self.roll_first = true;
        }

        Ok(self)
    }

    /// Implements checks to validate if the player is allowed to move
    fn move_permitted(&mut self, player: Player, dice: u8) -> Result<&mut Self, Error> {
        // check if player is allowed to move
        if player != self.who_plays {
            return Err(Error::NotYourTurn);
        }

        // if player is nobody, you can not play and have to roll first
        if self.who_plays == Player::Nobody {
            return Err(Error::RollFirst);
        }

        // check if player has to take or reject cube first
        if self.cube_received {
            return Err(Error::CubeReceived);
        }

        // check if player has to roll first
        if self.roll_first {
            return Err(Error::RollFirst);
        }

        // check if dice value has actually been rolled
        if dice != self.dices.values.0 && dice != self.dices.values.1 {
            return Err(Error::DiceInvalid);
        }

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

    // Test Display trait for Game
    #[test]
    fn test_display() {
        let g = Game::new();
        assert_eq!(
            format!("{}", g),
            "Rules: Points: 7, Beaver: false, Raccoon: false, Murphy: false, Murphy Limit: 0, Jacoby: false, Crawford: true, Holland: false\nDices: Dices { values: (0, 0), consumed: (false, false, false, false) }\nCube: 1\nCube owner: Nobody\nWho plays: Nobody\nBoard: BoardDisplay { board: [-2, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, -5, 5, 0, 0, 0, -3, 0, -5, 0, 0, 0, 0, 2], bar: (0, 0), off: (0, 0) }\nCrawford game: false\nSince Crawford game: 0\n"
        );
    }

    // Test GameRules trait for Game
    #[test]
    fn test_game_rules() {
        let g = Game::new()
            .with_beaver()
            .with_raccoon()
            .with_murphy(3)
            .with_jacoby()
            .with_holland();
        assert!(g.rules.beaver);
        assert!(g.rules.raccoon);
        assert!(g.rules.murphy);
        assert_eq!(g.rules.murphy_limit, 3);
        assert!(g.rules.jacoby);
        assert!(g.rules.holland);
    }
}
