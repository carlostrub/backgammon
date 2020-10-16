use super::CurrentRules;
use super::Rules;

impl Default for CurrentRules {
    fn default() -> Self {
        CurrentRules {
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

impl Rules for CurrentRules {
    fn with_beaver(self) -> Self {
        CurrentRules {
            beaver: true,
            ..self
        }
    }
    fn is_beaver(self) -> bool {
        self.beaver
    }

    fn with_raccoon(self) -> Self {
        CurrentRules {
            beaver: true,
            raccoon: true,
            ..self
        }
    }
    fn is_raccoon(self) -> bool {
        self.raccoon
    }

    fn with_murphy(self, limit: u8) -> Self {
        CurrentRules {
            murphy: true,
            murphy_limit: limit,
            ..self
        }
    }
    fn is_murphy(self) -> bool {
        self.murphy
    }

    fn with_jacoby(self) -> Self {
        CurrentRules {
            jacoby: true,
            ..self
        }
    }
    fn is_jacoby(self) -> bool {
        self.jacoby
    }

    fn with_crawford(self) -> Self {
        CurrentRules {
            crawford: true,
            ..self
        }
    }
    fn is_crawford(self) -> bool {
        self.crawford
    }

    fn with_holland(self) -> Self {
        CurrentRules {
            crawford: true,
            holland: true,
            ..self
        }
    }
    fn is_holland(self) -> bool {
        self.holland
    }
}
