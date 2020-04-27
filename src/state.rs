// State holds the situation and the accounting information
pub struct State {
    free_positions_computer: u32,
    free_positions_opponent: u32,
    accounting: Accounting,
}

/// Accounting holds the number of stones in each position and the level of the doubling cube
pub struct Accounting {
    // negative values represent Opponent's stones, number 25 is the Computer's bar, number 26 is
    // the Opponent's bar
    board: [i8; 26],

    // this displays the n-th power of 2, negative number means it is owned by the opponent
    cube: i8,
}

impl State {
    fn calculate_free_positions(&mut self) {
        // set free positions of computer to zero
        self.free_positions_computer = 0;
        self.free_positions_opponent = 0;

        // check bar first
        if self.accounting.board[24] > 0 {
            for i in 0..5 {
                if self.accounting.board[i] > -2 {
                    self.free_positions_computer |= 2u32.pow(i as u32);
                }
            }
        } else {
            for i in 0..23 {
                if self.accounting.board[i] > -2 {
                    self.free_positions_computer |= 2u32.pow(i as u32);
                }
            }


            if self.accounting.board[25] > 0 {
                // set free positions of computer to zer
                self.free_positions_opponent = 0;
                for i in 18..23 {
                    if self.accounting.board[i] > -1 {
                        self.free_positions_computer |= 2u32.pow(i as u32);
                    }
                }
            }



        }
    }
}

pub fn reset() -> State {
    let mut x = State {
        // TODO: free_position calculation
        free_positions_computer: 0,
        free_positions_opponent: 0,
        accounting: Accounting {
            board: [
                2,
                0,
                0,
                0,
                0,
                -5,
                0,
                -3,
                0,
                0,
                0,
                5,
                -5,
                0,
                0,
                0,
                3,
                0,
                5,
                0,
                0,
                0,
                0,
                -2,
                0,
                0,
            ],
            cube: 1,
        },
    };
    return x;
}
