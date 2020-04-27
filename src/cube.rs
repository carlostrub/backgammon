use std::io;

/// Rules hold different rules for the doubling cube
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

/// rules_valid performs a pre-check on whether the rules selected make sense
pub fn rules_valid(r: &Rules) -> Result<bool, io::Error> {
    if r.raccoon && !r.beaver {
        panic!("Raccoon rule only valid together with beaver rule.");
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_valid_test() {
        let d = Rules {
            beaver: true,
            raccoon: true,
            crawford: true,
            jacoby: true,
        };
        rules_valid(&d);
    }

    #[test]
    #[should_panic]
    fn rules_not_valid_test() {
        let d = Rules {
            beaver: false,
            raccoon: true,
            crawford: true,
            jacoby: true,
        };
        rules_valid(&d);
    }
}
