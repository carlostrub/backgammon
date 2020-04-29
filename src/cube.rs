use std::io;

/// Rules hold different rules for the doubling cube
#[derive(Debug)]
pub struct Rules {
    /// When offered the cube, allow to re-double but keep it.
    pub beaver: bool,
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    pub raccoon: bool,
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    pub jacoby: bool,
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    pub crawford: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            beaver: false,
            raccoon: false,
            jacoby: false,
            crawford: false,
        }
    }
}

impl Rules {
    /// rules_valid performs a pre-check on whether the rules selected make sense
    pub fn rules_valid(&self) -> Result<bool, io::Error> {
        if self.raccoon && !self.beaver {
            panic!("Raccoon rule only valid together with beaver rule.");
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {}
