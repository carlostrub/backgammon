use crate::Error;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

/// Represents the two dices
///
/// Backgammon is always played with two dices.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize, Default)]
pub struct Dices {
    /// The two dice values
    pub values: (u8, u8),
    /// Boolean indicating whether the dices have been consumed already. We use a tuple
    /// of four booleans in case the dices are equal, in which case we have four dices
    /// to play.
    pub consumed: (bool, bool, bool, bool),
}
impl Dices {
    /// Roll the dices which generates two random numbers between 1 and 6, replicating a perfect
    /// dice. We use the operating system's random number generator.
    pub fn roll(self) -> Self {
        let between = Uniform::new_inclusive(1, 6);
        let mut rng = rand::thread_rng();

        let v = (between.sample(&mut rng), between.sample(&mut rng));

        // if both dices are equal, we have four dices to play
        if v.0 == v.1 {
            Dices {
                values: (v.0, v.1),
                consumed: (false, false, false, false),
            }
        } else {
            Dices {
                values: (v.0, v.1),
                consumed: (false, false, true, true),
            }
        }
    }
}

/// Trait to roll the dices
pub trait Roll {
    /// Roll the dices
    fn roll(&mut self) -> Result<&mut Self, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let dices = Dices::default().roll();
        assert!(dices.values.0 >= 1 && dices.values.0 <= 6);
        assert!(dices.values.1 >= 1 && dices.values.1 <= 6);
    }

    #[test]
    fn test_roll_consumed() {
        let dices = Dices::default().roll();
        if dices.values.0 == dices.values.1 {
            assert_eq!(dices.consumed, (false, false, false, false));
        } else {
            assert_eq!(dices.consumed, (false, false, true, true));
        }
    }

    #[test]
    fn test_roll_consumed1() {
        for _i in 0..100 {
            let dices = Dices::default().roll();
            if dices.values.0 == dices.values.1 {
                assert_eq!(dices.consumed, (false, false, false, false));
            } else {
                assert_eq!(dices.consumed, (false, false, true, true));
            }
        }
    }
}
