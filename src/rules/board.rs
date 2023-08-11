use crate::rules::Player;
use crate::Error;
use serde::{Deserialize, Serialize};

/// Represents the Backgammon board
///
/// A Backgammon board consists of 24 fields, each of which can hold 0 or more checkers. In
/// addition there is a bar to hold checkers that have been hit and an off area to hold checkers
/// that have been removed from the board.
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Default)]
pub struct Board {
    raw_board: (PlayerBoard, PlayerBoard),
}

/// Represents the Backgammon board for both players (to be used for graphical representation).
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Default)]
pub struct BoardDisplay {
    /// The board represented as an array of 24 fields, each of which can hold 0 or more checkers.
    /// Positive amounts represent checkers of player 0, negative amounts represent checkers of
    /// player 1.
    pub board: [i8; 24],
    /// The bar for both players
    pub bar: (u8, u8),
    /// The off for both players
    pub off: (u8, u8),
}

impl Board {
    /// Create a new board
    pub fn new() -> Self {
        Board::default()
    }

    /// Get the board for both players. Use for graphical representation of the board.
    ///
    /// This method outputs a tuple with three values:
    ///
    /// 1. the board represented as an array of 24 fields, each of which can hold 0 or more
    /// checkers. Positive amounts represent checkers of player 0, negative amounts represent
    /// checkers of player 1.
    /// 2. the bar for both players
    /// 3. the off for both players
    pub fn get(&self) -> BoardDisplay {
        let mut board: [i8; 24] = [0; 24];

        for (i, val) in board.iter_mut().enumerate() {
            *val = self.raw_board.0.board[i] as i8 - self.raw_board.1.board[i] as i8;
        }

        BoardDisplay {
            board,
            bar: self.get_bar(),
            off: self.get_off(),
        }
    }

    /// Get the bar for both players
    fn get_bar(&self) -> (u8, u8) {
        (self.raw_board.0.bar, self.raw_board.1.bar)
    }

    /// Get the off for both players
    fn get_off(&self) -> (u8, u8) {
        (self.raw_board.0.off, self.raw_board.1.off)
    }

    /// Set checkers for a player on a field
    pub fn set(&mut self, player: Player, field: u8, amount: u8) -> Result<(), Error> {
        match player {
            Player::Player0 => {
                self.raw_board.0.board[field as usize] = amount;
                Ok(())
            }
            Player::Player1 => {
                self.raw_board.1.board[field as usize] = amount;
                Ok(())
            }
            Player::Nobody => Err(Error::InvalidPlayer),
        }
    }

    /// Set checkers for a player on the bar
    pub fn set_bar(&mut self, player: Player, amount: u8) -> Result<(), Error> {
        match player {
            Player::Player0 => {
                self.raw_board.0.bar = amount;
                Ok(())
            }
            Player::Player1 => {
                self.raw_board.1.bar = amount;
                Ok(())
            }
            Player::Nobody => Err(Error::InvalidPlayer),
        }
    }

    /// Set checkers for a player off the board
    pub fn set_off(&mut self, player: Player, amount: u8) -> Result<(), Error> {
        match player {
            Player::Player0 => {
                self.raw_board.0.off = amount;
                Ok(())
            }
            Player::Player1 => {
                self.raw_board.1.off = amount;
                Ok(())
            }
            Player::Nobody => Err(Error::InvalidPlayer),
        }
    }
}

/// Represents the Backgammon board for one player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PlayerBoard {
    board: [u8; 24],
    bar: u8,
    off: u8,
}

impl Default for PlayerBoard {
    fn default() -> Self {
        PlayerBoard {
            board: [
                0, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            ],
            bar: 0,
            off: 0,
        }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_board() {
        assert_eq!(Board::new(), Board::default());
    }

    #[test]
    fn default_player_board() {
        assert_eq!(
            PlayerBoard::default(),
            PlayerBoard {
                board: [0, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,],
                bar: 0,
                off: 0
            }
        );
    }

    #[test]
    fn default_board_display() {
        assert_eq!(
            BoardDisplay::default(),
            BoardDisplay {
                board: [0, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,],
                bar: (0, 0),
                off: (0, 0)
            }
        );
    }

    #[test]
    fn get_board() {
        let board = Board::new();
        assert_eq!(
            board.get(),
            BoardDisplay {
                board: [0, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,],
                bar: (0, 0),
                off: (0, 0)
            }
        );
    }

    #[test]
    fn get_bar() {
        let board = Board::new();
        assert_eq!(board.get_bar(), (0, 0));
    }

    #[test]
    fn get_off() {
        let board = Board::new();
        assert_eq!(board.get_off(), (0, 0));
    }

    #[test]
    fn set_player0() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 0, 1)?;
        assert_eq!(board.get().board[0], 1);
        Ok(())
    }

    #[test]
    fn set_player1() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player1, 0, 1)?;
        assert_eq!(board.get().board[23], -1);
        Ok(())
    }

    #[test]
    fn set_player0_bar() -> Result<(), Error> {
        let mut board = Board::new();
        board.set_bar(Player::Player0, 1)?;
        assert_eq!(board.get().bar.0, 1);
        Ok(())
    }

    #[test]
    fn set_player1_bar() -> Result<(), Error> {
        let mut board = Board::new();
        board.set_bar(Player::Player1, 1)?;
        assert_eq!(board.get().bar.1, 1);
        Ok(())
    }

    #[test]
    fn set_player0_off() -> Result<(), Error> {
        let mut board = Board::new();
        board.set_off(Player::Player0, 1)?;
        assert_eq!(board.get().off.0, 1);
        Ok(())
    }

    #[test]
    fn set_player1_off() -> Result<(), Error> {
        let mut board = Board::new();
        board.set_off(Player::Player1, 1)?;
        assert_eq!(board.get().off.1, 1);
        Ok(())
    }

    #[test]
    fn set_invalid_player() {
        let mut board = Board::new();
        assert!(board.set(Player::Nobody, 0, 1).is_err());
        assert!(board.set_bar(Player::Nobody, 1).is_err());
        assert!(board.set_off(Player::Nobody, 1).is_err());
    }
}
