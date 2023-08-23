use bevy::prelude::*;

#[derive(Component, Resource, Debug, PartialEq, Eq, Default, Copy, Clone)]
pub enum GameType {
    #[default]
    None,
    Normal,
    SpockLizard,
    FireWater,
}

impl GameType {
    pub fn max_number_of_moves(&self) -> i32 {
        match self {
            GameType::Normal => 3,
            _ => 5,
        }
    }

    pub fn get_friendly_name(&self) -> &str {
        match self {
            GameType::Normal => "rock paper scissors",
            GameType::SpockLizard => "Spock lizard variation",
            GameType::FireWater => "fire water variation",
            _ => "None",
        }
    }
}

impl From<i32> for GameType {
    fn from(value: i32) -> Self {
        match value {
            1 => GameType::Normal,
            2 => GameType::SpockLizard,
            3 => GameType::FireWater,
            _ => GameType::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_type::GameType;

    #[test]
    fn test_max_number_of_moves() {
        assert_eq!(GameType::Normal.max_number_of_moves(), 3);
        assert_eq!(GameType::SpockLizard.max_number_of_moves(), 5);
        assert_eq!(GameType::FireWater.max_number_of_moves(), 5);
    }

    #[test]
    fn test_get_friendly_name() {
        assert_eq!(GameType::Normal.get_friendly_name(), "rock paper scissors");
        assert_eq!(GameType::SpockLizard.get_friendly_name(), "Spock lizard variation");
        assert_eq!(GameType::FireWater.get_friendly_name(), "fire water variation");
        assert_eq!(GameType::None.get_friendly_name(), "None");
    }

    #[test]
    fn test_i32_into_game_type() {
        assert_eq!(Into::<GameType>::into(1), GameType::Normal);
        assert_eq!(Into::<GameType>::into(2), GameType::SpockLizard);
        assert_eq!(Into::<GameType>::into(3), GameType::FireWater);
        assert_eq!(Into::<GameType>::into(10), GameType::None);
    }
}
