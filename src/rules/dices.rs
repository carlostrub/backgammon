use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

/// Represents the two dices
///
/// Backgammon is always played with two dices.
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Default)]
pub struct Dices(pub u8, pub u8);

impl Dices {
    /// Roll the dices which generates two random numbers between 1 and 6, replicating a perfect
    /// dice. We use the operating system's random number generator.
    pub fn roll(self) -> Self {
        let between = Uniform::new_inclusive(1, 6);
        let mut rng = rand::thread_rng();

        Dices(between.sample(&mut rng), between.sample(&mut rng))
    }
}
