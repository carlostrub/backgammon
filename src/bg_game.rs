use rand::distributions::{Distribution, Uniform};
/// Represents a Backgammon game
#[derive(Debug, Clone, Copy)]
pub struct Game {
    /// how many points in the game?
    pub points: u32,
    /// who is the winner?
    pub winner: Player,
    /// last dice pair rolled
    pub dices: (u8, u8),
    /// whose turn is it?
    pub who_plays: Player,
    /// a board has 24 fields, the second tuple is the bar for Player 1 and 2, the third tuple is
    /// the off for Player 1 and 2
    pub board: ([i8; 24], (u8, u8), (u8, u8)),
    /// cube displays the n-th power of 2, e.g. 2 -> 2^2 = 4
    pub cube: u8,
    /// who holds the cube
    pub cube_owner: Player,
    /// was cube offered to the one who plays?
    pub cube_received: bool,
    // Crawford rule: if crawford game, no doubling allowed
    crawford: bool,
    // Holland rule: if <4 rolls of crawford game, no doubling allowed
    since_crawford: u8,
}

// Backgammon uses 15 checkers per side
//const CHECKERS: u8 = 15;

impl Game {
    /// Start a new game
    pub fn new() -> Self {
        Game::default()
    }

    /// Roll the dices which generates two random numbers between 1 and 6, replicating a perfect
    /// dice. We use the operating systems random number generator.
    pub fn roll(mut self, p: Player) -> Result<Self, Error> {
        let between = Uniform::new_inclusive(1, 6);
        let mut rng = rand::thread_rng();

        match self.who_plays == p || self.who_plays == Player::Nobody {
            false => Err(Error::TurnError),
            true => match self.cube_received {
                true => Err(Error::DiceReceivedError),
                false => {
                    self.dices = (between.sample(&mut rng), between.sample(&mut rng));
                    Ok(self)
                }
            },
        }
    }

    /// Start game by rolling a pair of different numbers to define who begins.
    pub fn start(self) -> Result<Self, Error> {
        loop {
            let g = self.roll(Player::Nobody);
            match g {
                Ok(mut g) => {
                    if g.dices.0 != g.dices.1 {
                        if g.dices.0 > g.dices.1 {
                            g.who_plays = Player::Player1;
                        } else {
                            g.who_plays = Player::Player2;
                        }
                        return Ok(g);
                    } else {
                        continue;
                    }
                }
                Err(_) => return Err(Error::StartedError),
            }
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

impl Default for Game {
    fn default() -> Self {
        Game {
            points: 0,
            winner: Player::Nobody,
            dices: (0, 0),
            cube: 0,
            cube_owner: Player::Nobody,
            who_plays: Player::Nobody,
            board: (
                [
                    2, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2,
                ],
                (0, 0),
                (0, 0),
            ),
            cube_received: false,
            crawford: false,
            since_crawford: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_test() {
        let g = Game::default();
        let d = g.roll(Player::Player1).unwrap();
        assert!(d.dices.0 > 0);
        assert!(d.dices.0 < 7);
        assert!(d.dices.1 > 0);
        assert!(d.dices.1 < 7);
    }

    #[test]
    fn roll_test_fair() {
        let mut sum: u32 = 0;

        for _x in 0..1_000_000 {
            let g = Game::default();
            let d = g.roll(Player::Player1).unwrap();
            sum += (d.dices.0 + d.dices.1) as u32;
        }

        let average = (sum as f64) / 2_000_000.;
        assert!(average < 3.51);
        assert!(average > 3.49);
    }

    #[test]
    fn start_test() {
        let g = Game::default();
        for _x in 0..1_000_000 {
            let d = g.start().unwrap();
            assert!(d.dices.0 != d.dices.1);
        }
    }
}
