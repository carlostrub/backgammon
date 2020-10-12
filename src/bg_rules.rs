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
            crawford: false,
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

    fn with_raccoon(self) -> Self {
        CurrentRules {
            beaver: true,
            raccoon: true,
            ..self
        }
    }

    fn with_murphy(self, limit: u8) -> Self {
        CurrentRules {
            murphy: true,
            murphy_limit: limit,
            ..self
        }
    }

    fn with_jacoby(self) -> Self {
        CurrentRules {
            jacoby: true,
            ..self
        }
    }

    fn with_crawford(self) -> Self {
        CurrentRules {
            crawford: true,
            ..self
        }
    }

    fn with_holland(self) -> Self {
        CurrentRules {
            crawford: true,
            holland: true,
            ..self
        }
    }
}
