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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_invalid_rules_test() {
        let r = Rules {
            beaver: false,
            raccoon: true,
            crawford: true,
            jacoby: true,
        };
        let p = (Player::default(), Player::default());

        New(13, p, r);
    }

    #[test]
    #[should_panic]
    fn new_invalid_player0_test() {
        let r = Rules {
            beaver: false,
            raccoon: false,
            crawford: false,
            jacoby: false,
        };
        let p = (Player::default(), Player::default());

        New(13, p, r);
    }

    #[test]
    #[should_panic]
    fn new_invalid_player1_test() {
        let r = Rules {
            beaver: false,
            raccoon: false,
            crawford: false,
            jacoby: false,
        };
        let p = (Player::default(), Player::default());

        New(13, p, r);
    }
}
