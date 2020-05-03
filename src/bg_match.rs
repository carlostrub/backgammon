use std::time::SystemTime;

use super::Match;
use super::Player;
use super::Rules;

use uuid::Uuid;

impl Default for Match {
    fn default() -> Self {
        let id = Uuid::new_v4();
        let time_start = SystemTime::now();
        let players = (Player::default(), Player::default());

        Match {
            id,
            points: 0,
            rules: Rules::default(),
            history: Vec::new(),
            players,
            time_start,
            time_end: time_start,
        }
    }
}

/// Implements methods for the Match struct
impl Match {
    /// Create a new Match
    pub fn new() -> Self {
        Match::default()
    }
}

#[cfg(test)]
mod tests {}
