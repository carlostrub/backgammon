use std::io;

/// Player holds informations about a game player.
pub struct Player {
    pub name: String,
}

/// player_valid performs a pre-check on whether the player selected has no missing values
pub fn player_valid(p: &Player) -> Result<bool, io::Error> {
    if p.name == "" {
        panic!("No name set.");
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_valid_test() {
        let p = Player { name: String::from("Carlo Strub") };
        player_valid(&p);
    }

    #[test]
    #[should_panic]
    fn player_not_valid_test() {
        let p = Player { name: String::from("") };
        player_valid(&p);
    }
}
