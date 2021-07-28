use rand::distributions::{Distribution, Uniform};

use super::{Game, Player, Statistics};

// Backgammon uses 15 checkers per side
//const CHECKERS: u8 = 15;

impl Game {
    /// Start a new game
    pub fn new() -> Self {
        Game::default()
    }

    /// roll generates two random numbers between 1 and 6, replicating a perfect dice. We use the
    /// operating systems random number generator.
    pub fn roll(&mut self) {
        let between = Uniform::new_inclusive(1, 6);
        let mut rng = rand::thread_rng();

        self.dices = (between.sample(&mut rng), between.sample(&mut rng))
    }

    /// start game by rolling a pair of different numbers to define who begins
    pub fn start(mut self) -> Result<Self, &'static str> {
        match self.who_plays {
            Player::Nobody => {
                loop {
                    self.roll();
                    if self.dices.0 != self.dices.1 {
                        break;
                    };
                }
                if self.dices.0 > self.dices.1 {
                    self.who_plays = Player::Player1;
                } else {
                    self.who_plays = Player::Player2;
                }
                Ok(self)
            }
            Player::Player1 => Err("game already running"),
            Player::Player2 => Err("game already running"),
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
            statistics: Statistics::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_test() {
        let d = roll();
        assert!(d.0 > 0);
        assert!(d.0 < 7);
        assert!(d.1 > 0);
        assert!(d.1 < 7);
    }

    #[test]
    fn roll_test_fair() {
        let mut sum: u32 = 0;

        for _x in 0..1_000_000 {
            let d = roll();
            sum += (d.0 + d.1) as u32;
        }

        let average = (sum as f64) / 2_000_000.;
        assert!(average < 3.51);
        assert!(average > 3.49);
    }
}
