use super::Rules;

impl Default for Rules {
    fn default() -> Self {
        Rules {
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
