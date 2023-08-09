use crate::error::Error;
use crate::rules::player::Player;
use serde::{Deserialize, Serialize};

/// Represents a Backgammon cube (doubling cube).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Cube {
    exponential: u8,
    /// Owner of the cube
    owner: Player,
}

impl Cube {
    /// Returns the value of the cube
    pub fn value(&self) -> u64 {
        2 ^ self.exponential as u64
    }

    /// Returns the owner of the cube
    pub fn owner(&self) -> Player {
        self.owner
    }

    /// Set the value of the cube
    pub fn set(&mut self, value: u64) -> Result<(), Error> {
        if value.is_power_of_two() {
            self.exponential = value.trailing_zeros() as u8;
            Ok(())
        } else {
            Err(Error::InvalidCubeValue)
        }
    }

    /// Calculate the next value of the cube to offer it to the opponent
    pub fn offer(&self, opponent: Player) -> Result<u64, Error> {
        if self.owner == Player::Nobody || self.owner != opponent {
            Ok(2 ^ self.exponential as u64)
        } else {
            Err(Error::Double)
        }
    }
}
