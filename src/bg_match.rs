use std::time::SystemTime;

use super::cube::Rules;
use super::game::Game;
use super::player::Player;

use uuid::Uuid;

/// Match holds informations about a match
#[derive(Debug)]
pub struct Match {
    id: Uuid,
    points: u32,
    rules: Rules,
    history: Vec<Game>,
    players: (Player, Player),
    time_start: SystemTime,
    time_end: SystemTime,
}

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
