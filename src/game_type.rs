use bevy::prelude::*;

#[derive(Component, Resource, Debug, PartialEq, Eq, Default, Copy, Clone)]
pub enum GameType {
    #[default]
    None,
    Normal,
    SpockLizard,
    FireWater
}
impl GameType {
    pub fn max_number_of_move(&self) -> i32 {
        match self {
            GameType::Normal => 3,
            _ => 5
        }
    }

    pub fn get_friendly_title(&self) -> &str {
        match self {
            GameType::Normal => "rock paper scissors",
            GameType::SpockLizard => "Spock lizard variation",
            GameType::FireWater => "fire water variation",
            _ => "None"
        }
    }
}
impl Into<GameType> for i32 {
    fn into(self) -> GameType {
        match self {
            1 => GameType::Normal,
            2 => GameType::SpockLizard,
            3 => GameType::FireWater,
            _ => GameType::None,
        }
    }
}