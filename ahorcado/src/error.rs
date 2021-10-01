use std::fmt;

#[derive(Debug)]
pub enum GameError {
    NoChancesAvailable,
    CharacterIsAlreadyUsed,
}

impl std::error::Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::NoChancesAvailable => write!(f, "No more chances"),
            GameError::CharacterIsAlreadyUsed => write!(f, "Character is already used"),
        }
    }
}