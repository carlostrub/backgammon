use crate::rules::Player;
use crate::Error;
use serde::{Deserialize, Serialize};

/// Represents the Backgammon board
///
/// A Backgammon board consists of 24 fields, each of which can hold 0 or more checkers. In
/// addition there is a bar to hold checkers that have been hit and an off area to hold checkers
/// that have been removed from the board.
///
/// ```
/// # fn foo() {}
/// //        +12-11-10--9--8--7-------6--5--4--3--2--1-+
/// //        | X           O    |   | O              X | +-------+
/// //        | X           O    |   | O              X | | OFF O |
/// //        | X           O    |   | O                | +-------+
/// //        | X                |   | O                |
/// //        | X                |   | O                |
/// //        |                  |BAR|                  |
/// //        | O                |   | X                |
/// //        | O                |   | X                |
/// //        | O           X    |   | X                | +-------+
/// //        | O           X    |   | X              O | | OFF X |
/// //        | O           X    |   | X              O | +-------+
/// //        +13-14-15-16-17-18------19-20-21-22-23-24-+
/// ```

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Default)]
pub struct Board {
    raw_board: (PlayerBoard, PlayerBoard),
}

/// Represents the Backgammon board for both players (to be used for graphical representation).
#[derive(Debug, Serialize, PartialEq, Deserialize)]
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
            *val = self.raw_board.0.board[i] as i8 - self.raw_board.1.board[23 - i] as i8;
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
    ///
    /// This method adds the amount of checkers for a player on a field. The field is numbered from
    /// 0 to 23, starting from the last field of each player in the home board, the most far away
    /// field for each player (where there are 2 checkers to start with) is number 23.
    ///
    /// If the field is blocked for the player, an error is returned. If the field is not blocked,
    /// but there is already one checker from the other player on the field, that checker is hit and
    /// moved to the bar.
    pub fn set(&mut self, player: Player, field: usize, amount: i8) -> Result<(), Error> {
        if field > 23 {
            return Err(Error::FieldInvalid);
        }

        if self.blocked(player, field)? {
            return Err(Error::FieldBlocked);
        }

        match player {
            Player::Player0 => {
                let new = self.raw_board.0.board[field] as i8 + amount;
                if new < 0 {
                    return Err(Error::MoveInvalid);
                }
                self.raw_board.0.board[field] = new as u8;

                // in case one opponent's checker is hit, move it to the bar
                self.raw_board.1.bar += self.raw_board.1.board[23 - field];
                self.raw_board.1.board[23 - field] = 0;
                Ok(())
            }
            Player::Player1 => {
                let new = self.raw_board.1.board[field] as i8 + amount;
                if new < 0 {
                    return Err(Error::MoveInvalid);
                }
                self.raw_board.1.board[field] = new as u8;

                // in case one opponent's checker is hit, move it to the bar
                self.raw_board.0.bar += self.raw_board.0.board[23 - field];
                self.raw_board.0.board[23 - field] = 0;
                Ok(())
            }
            Player::Nobody => Err(Error::PlayerInvalid),
        }
    }

    /// Check if a field is blocked for a player
    pub fn blocked(&self, player: Player, field: usize) -> Result<bool, Error> {
        if field > 23 {
            return Err(Error::FieldInvalid);
        }

        match player {
            Player::Player0 => {
                if self.raw_board.1.board[23 - field] > 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Player::Player1 => {
                if self.raw_board.0.board[23 - field] > 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Player::Nobody => Err(Error::PlayerInvalid),
        }
    }

    /// Set checkers for a player on the bar. This method adds amount to the already existing
    /// checkers there.
    pub fn set_bar(&mut self, player: Player, amount: i8) -> Result<(), Error> {
        match player {
            Player::Player0 => {
                let new = self.raw_board.0.bar as i8 + amount;
                if new < 0 {
                    return Err(Error::MoveInvalid);
                }
                self.raw_board.0.bar = new as u8;
                Ok(())
            }
            Player::Player1 => {
                let new = self.raw_board.1.bar as i8 + amount;
                if new < 0 {
                    return Err(Error::MoveInvalid);
                }
                self.raw_board.1.bar = new as u8;
                Ok(())
            }
            Player::Nobody => Err(Error::PlayerInvalid),
        }
    }

    /// Set checkers for a player off the board. This method adds amount to the already existing
    /// checkers there.
    pub fn set_off(&mut self, player: Player, amount: u8) -> Result<(), Error> {
        match player {
            Player::Player0 => {
                let new = self.raw_board.0.off + amount;
                self.raw_board.0.off = new;
                Ok(())
            }
            Player::Player1 => {
                let new = self.raw_board.1.off + amount;
                self.raw_board.1.off = new;
                Ok(())
            }
            Player::Nobody => Err(Error::PlayerInvalid),
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

/// Trait to move checkers
pub trait Move {
    /// Move a checker
    fn move_checker(&mut self, player: Player, dice: u8, from: usize) -> Result<&mut Self, Error>
    where
        Self: Sized;

    /// Move a checker from bar
    fn move_checker_from_bar(&mut self, player: Player, dice: u8) -> Result<&mut Self, Error>
    where
        Self: Sized;

    /// Move permitted
    fn move_permitted(&mut self, player: Player, dice: u8) -> Result<&mut Self, Error>
    where
        Self: Sized;
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
    fn get_board() {
        let board = Board::new();
        assert_eq!(
            board.get(),
            BoardDisplay {
                board: [
                    -2, 0, 0, 0, 0, 5, 0, 3, 0, 0, 0, -5, 5, 0, 0, 0, -3, 0, -5, 0, 0, 0, 0, 2,
                ],
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
        board.set(Player::Player0, 1, 1)?;
        assert_eq!(board.get().board[1], 1);
        Ok(())
    }

    #[test]
    fn set_player1() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player1, 2, 1)?;
        assert_eq!(board.get().board[21], -1);
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
    fn set_player1_off1() -> Result<(), Error> {
        let mut board = Board::new();
        board.set_off(Player::Player1, 1)?;
        board.set_off(Player::Player1, 1)?;
        assert_eq!(board.get().off.1, 2);
        Ok(())
    }

    #[test]
    fn set_invalid_player() {
        let mut board = Board::new();
        assert!(board.set(Player::Nobody, 0, 1).is_err());
        assert!(board.set_bar(Player::Nobody, 1).is_err());
        assert!(board.set_off(Player::Nobody, 1).is_err());
    }

    #[test]
    fn blocked_player0() -> Result<(), Error> {
        let board = Board::new();
        assert!(board.blocked(Player::Player0, 0)?);
        Ok(())
    }

    #[test]
    fn blocked_player1() -> Result<(), Error> {
        let board = Board::new();
        assert!(board.blocked(Player::Player1, 0)?);
        Ok(())
    }

    #[test]
    fn blocked_player0_a() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player1, 1, 2)?;
        assert!(board.blocked(Player::Player0, 22)?);
        Ok(())
    }

    #[test]
    fn blocked_player1_a() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 1, 2)?;
        assert!(board.blocked(Player::Player1, 22)?);
        Ok(())
    }

    #[test]
    fn blocked_invalid_player() {
        let board = Board::new();
        assert!(board.blocked(Player::Nobody, 0).is_err());
    }

    #[test]
    fn blocked_invalid_field() {
        let board = Board::new();
        assert!(board.blocked(Player::Player0, 24).is_err());
    }

    #[test]
    fn set_field_with_1_checker_player0_a() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 1, 1)?;
        board.set(Player::Player1, 22, 1)?;
        assert_eq!(board.get().board[1], -1);
        assert_eq!(board.get().bar.0, 1);
        Ok(())
    }

    #[test]
    fn set_field_with_1_checker_player0_b() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 1, 1)?;
        board.set_bar(Player::Player0, 5)?;
        board.set(Player::Player1, 22, 1)?;
        assert_eq!(board.get().board[1], -1);
        assert_eq!(board.get().bar.0, 6);
        Ok(())
    }

    #[test]
    fn set_field_with_1_checker_player1_a() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player1, 1, 1)?;
        board.set(Player::Player0, 22, 1)?;
        assert_eq!(board.get().board[22], 1);
        assert_eq!(board.get().bar.1, 1);
        Ok(())
    }

    #[test]
    fn set_field_with_1_checker_player1_b() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player1, 1, 1)?;
        board.set_bar(Player::Player1, 5)?;
        board.set(Player::Player0, 22, 1)?;
        assert_eq!(board.get().board[22], 1);
        assert_eq!(board.get().bar.1, 6);
        Ok(())
    }

    #[test]
    fn set_field_with_2_checkers_player0_a() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 23, 2)?;
        assert_eq!(board.get().board[23], 4);
        Ok(())
    }

    #[test]
    fn set_field_with_2_checkers_player0_b() -> Result<(), Error> {
        let mut board = Board::new();
        board.set(Player::Player0, 23, -1)?;
        assert_eq!(board.get().board[23], 1);
        Ok(())
    }

    #[test]
    fn set_field_blocked() {
        let mut board = Board::new();
        assert!(board.set(Player::Player0, 0, 2).is_err());
    }

    #[test]
    fn set_wrong_field1() {
        let mut board = Board::new();
        assert!(board.set(Player::Player0, 50, 2).is_err());
    }

    #[test]
    fn set_wrong_amount0() {
        let mut board = Board::new();
        assert!(board.set(Player::Player0, 23, -3).is_err());
    }

    #[test]
    fn set_wrong_amount1() {
        let mut board = Board::new();
        assert!(board.set(Player::Player1, 0, -3).is_err());
    }
}
