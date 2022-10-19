///
/// Holds the rules of the match
#[derive(Debug)]
struct Rules {
    /// The amount of points to reach for declaring a winner
    points: u32,
    /// When offered the cube, allow to re-double but keep it.
    beaver: bool,
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    raccoon: bool,
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    murphy: bool,
    /// How often to apply automatic doubling rule. 0 means always on.
    murphy_limit: u8,
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    jacoby: bool,
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    crawford: bool,
    /// Permits to double after Crawford game only if both players have rolled at least twice
    holland: bool,
}

/// Implements the Backgammon rules
pub trait Match {
    /// When offered the cube, allow to re-double but keep it.
    fn set_beaver(&self) -> Box<dyn Rules>;
    fn unset_beaver(&self) -> Box<dyn Rules>;
    /// Return true if beaver rule is set
    fn is_beaver(&self) -> bool;
    /// If a player plays "beaver", the other may double again, letting the opponent keep the cube.
    fn set_raccoon(&self) -> Box<dyn Rules>;
    fn unset_raccoon(&self) -> Box<dyn Rules>;
    /// Return true if Raccoon rule is set
    fn is_raccoon(&self) -> bool;
    /// If both players roll the same opening number, the dice is doubled, remaining in the middle
    /// of the board
    fn set_murphy(&self, limit: u8) -> Box<dyn Rules>;
    fn unset_murphy(&self, limit: u8) -> Box<dyn Rules>;
    /// Return true if Murphy rule is set
    fn is_murphy(&self) -> bool;
    /// Gammon and Backgammon only count for double or triple values if the cube has already been
    /// offered.
    fn set_jacoby(&self) -> Box<dyn Rules>;
    fn unset_jacoby(&self) -> Box<dyn Rules>;
    /// Return true if Jacoby rule is set
    fn is_jacoby(&self) -> bool;
    /// When a player first reaches a score of points - 1, no doubling is allowed for the following
    /// game.
    fn set_crawford(&self) -> Box<dyn Rules>;
    fn unset_crawford(&self) -> Box<dyn Rules>;
    /// Return true if Crawford rule is set
    fn is_crawford(&self) -> bool;
    /// Permits to double after Crawford game only if both players have rolled at least twice
    fn set_holland(&self) -> Box<dyn Rules>;
    fn unset_holland(&self) -> Box<dyn Rules>;
    /// Return true if Holland rule is set
    fn is_holland(&self) -> bool;
}


/// This enum is used in several places, e.g. for cube ownership or for winner
#[derive(Debug, PartialEq, Clone, Copy)]


impl Default for Box<dyn Rules> {
    fn default() -> Box<dyn Rules> {
        Box<dyn Rules> {
            beaver: false,
            raccoon: false,
            murphy: false,
            murphy_limit: 0,
            jacoby: false,
            crawford: true,
            holland: false,
        }
    }
}

impl Rules for box<Rules> {
    fn with_beaver(self) -> Box<dyn Rules> {
        BGRules {
            beaver: true,
            ..self
        }
    }
    fn is_beaver(&self) -> bool {
        self.beaver
    }

    fn with_raccoon(self) -> Box<dyn Rules> {
        Box<dyn BGRules> {
            beaver: true,
            raccoon: true,
            ..self
        }
    }
    fn is_raccoon(&self) -> bool {
        self.raccoon
    }

    fn with_murphy(self, limit: u8) -> Box<dyn Rules> {
        Box<dyn BGRules>{
            murphy: true,
            murphy_limit: limit,
            ..self
        }
    }
    fn is_murphy(&self) -> bool {
        self.murphy
    }

    fn with_jacoby(self) -> Box<dyn Rules> {
        Box<dyn BGRules> {
            jacoby: true,
            ..self
        }
    }
    fn is_jacoby(&self) -> bool {
        self.jacoby
    }

    fn with_crawford(self) -> Box<dyn Rules> {
        Box<dyn BGRules> {
            crawford: true,
            ..self
        }
    }
    fn is_crawford(&self) -> bool {
        self.crawford
    }

    fn with_holland(self) -> Box<dyn Rules> {
        Box<dyn BGRules> {
            crawford: true,
            holland: true,
            ..self
        }
    }
    fn is_holland(&self) -> bool {
        self.holland
    }
}
