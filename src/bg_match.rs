use std::io;
use std::time::SystemTime;

use cube::{rules_valid, Rules};
use game::Game;
use player::{player_valid, Player};

use uuid::Uuid;

/// Match holds informations about a match
pub struct Match {
    id: Uuid,
    points: u32,
    rules: Rules,
    history: Vec<Game>,
    players: (Player, Player),
    time_start: SystemTime,
    time_end: SystemTime,
}

/// New creates a new backgammon match between two players
pub fn New(points: u32, players: (Player, Player), rules: Rules) -> Result<Match, io::Error> {
    let rules_pre_check = rules_valid(&rules);

    match rules_pre_check {
        Ok(ok) => ok,
        Err(error) => panic!("Pre-check of rules lead to following error: {:?}", error),
    };

    let id = Uuid::new_v4();
    let time_start = SystemTime::now();

    let player_pre_check = player_valid(&players.0);

    match player_pre_check {
        Ok(ok) => ok,
        Err(error) => panic!("Pre-check of rules lead to following error: {:?}", error),
    };

    return Ok(Match {
        id: id,
        points: points,
        rules: rules,
        history: Vec::new(),
        players: players,
        time_start: time_start,
        time_end: time_start,
    });
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
        let p = (
            Player {
                name: "Carlo Strub".to_string(),
            },
            Player {
                name: "Cristina Strub".to_string(),
            },
        );

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
        let p = (
            Player {
                name: "".to_string(),
            },
            Player {
                name: "Cristina Strub".to_string(),
            },
        );

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
        let p = (
            Player {
                name: "Cristina Strub".to_string(),
            },
            Player {
                name: "".to_string(),
            },
        );

        New(13, p, r);
    }
}
