use serde::{Deserialize, Serialize};

/// Represents a Backgammon board for one player
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Represents a Backgammon board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    raw_board: (PlayerBoard, PlayerBoard),
}

impl Default for Board {
    fn default() -> Self {
        Board {
            raw_board: (PlayerBoard::default(), PlayerBoard::default()),
        }
    }
}

impl Board {
    /// Create a new board
    pub fn new() -> Self {
        Board::default()
    }

    /// Get the board for both players
    pub fn get(&self) -> [u8; 24] {
        let mut board = [0; 24];

        for i in 0..24 {
            board[i] = self.raw_board.0.board[i] + self.raw_board.1.board[23 - i];
        }

        board
    }
}
