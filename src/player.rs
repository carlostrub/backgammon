use std::fmt;
use uuid::Uuid;

/// Player holds informations about a backgammon player.
#[derive(Debug, Clone)]
pub struct Player {
    /// id of the player
    pub id: Uuid,
    /// Name of the player
    pub name: String,
}

impl Default for Player {
    fn default() -> Self {
        let uuid = Uuid::new_v4();

        Player {
            id: uuid,
            name: "Anonymous".to_string(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// PartialEq is implemented by comparing public keys and ids
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Eq is implemented using empty impl block
impl Eq for Player {}

/// Implements methods for the Player struct
impl Player {
    /// Return the identifier of a Player
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Change the name of a player
    pub fn with_name(self, name: String) -> Player {
        Player { name, ..self }
    }

    /// Create a new Player
    ///
    /// # Example
    ///
    /// ```
    /// use backgammon::player::Player;
    /// let player = Player::new().with_name("Carlo Strub".to_string());
    ///
    /// assert_eq!(format!("This is player {}", player), "This is player Carlo Strub");
    /// ```
    pub fn new() -> Self {
        Player::default()
    }
}
