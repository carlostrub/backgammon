use crate::error::Error;
use crate::rules::Player;
use serde::{Deserialize, Serialize};

/// Represents a Backgammon cube (doubling cube).
///
/// This cube represents an increase in the value of the current game. The cube -- a doubling of
/// the value of the game -- can be offered by any player the first time it is used. After that, it
/// can only be offered by the player who last took the cube.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Cube {
    exponential: u8,
    /// Owner of the cube
    owner: Player,
}

impl Cube {
    /// Returns the value of the cube
    pub fn value(&self) -> u64 {
        2u64.pow(self.exponential as u32)
    }

    /// Returns the owner of the cube
    pub fn owner(&self) -> Player {
        self.owner
    }

    /// Set the value of the cube.
    ///
    /// Internally, we store the value of the cube as an exponent of 2 as u8. Therefore there is a
    /// technical limit of 2^64 on the value of the cube, which we believe is a reasonable limit.
    pub fn set(&mut self, value: u64) -> Result<(), Error> {
        if value.is_power_of_two() {
            let vf = value as f64;
            self.exponential = vf.sqrt().round() as u8;

            Ok(())
        } else {
            Err(Error::CubeValueInvalid)
        }
    }

    /// Set owner of cube.
    pub fn set_owner(&mut self, owner: Player) {
        self.owner = owner;
    }

    /// Calculate the next value of the cube to offer it to the opponent
    pub fn offer(&self, opponent: Player) -> Result<u64, Error> {
        if self.owner == Player::Nobody || self.owner != opponent {
            Ok(2u64.pow(1 + self.exponential as u32))
        } else {
            Err(Error::DoubleNotPermitted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_value() {
        let cube = Cube::default();
        assert_eq!(cube.value(), 1);
    }

    #[test]
    fn set_value2() -> Result<(), Error> {
        let mut cube = Cube::default();
        cube.set(2)?;
        assert_eq!(cube.value(), 2);
        Ok(())
    }

    #[test]
    fn set_value4() -> Result<(), Error> {
        let mut cube = Cube::default();
        cube.set(4)?;
        assert_eq!(cube.value(), 4);
        Ok(())
    }

    #[test]
    fn set_value8() -> Result<(), Error> {
        let mut cube = Cube::default();
        cube.set(8)?;
        assert_eq!(cube.value(), 8);
        Ok(())
    }

    #[test]
    fn set_value16() -> Result<(), Error> {
        let mut cube = Cube::default();
        cube.set(16)?;
        assert_eq!(cube.value(), 16);
        Ok(())
    }

    #[test]
    fn set_invalidvalue() -> Result<(), Error> {
        let mut cube = Cube::default();
        assert!(cube.set(3).is_err());
        Ok(())
    }

    #[test]
    fn owner() {
        let cube = Cube::default();
        assert_eq!(cube.owner(), Player::Nobody);
    }

    #[test]
    fn owner1() {
        let mut cube = Cube::default();
        cube.set_owner(Player::Player0);
        assert_eq!(cube.owner(), Player::Player0);
    }

    #[test]
    fn offer0() -> Result<(), Error> {
        let cube = Cube::default();
        let offer = cube.offer(Player::Player1)?;
        assert_eq!(offer, 2);
        Ok(())
    }

    #[test]
    fn offer1() -> Result<(), Error> {
        let mut cube = Cube::default();
        cube.set(2)?;
        cube.set_owner(Player::Player0);
        let offer = cube.offer(Player::Player1)?;
        assert_eq!(cube.owner(), Player::Player0);
        assert_eq!(cube.value(), 2);
        assert_eq!(offer, 4);
        Ok(())
    }
}
