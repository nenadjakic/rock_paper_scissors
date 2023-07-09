use crate::game_error::GameError;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::game_move::GameMove::{FIRE, LIZARD, PAPER, QUIT, ROCK, SCISSORS, SPOCK, WATER};

pub trait MoveTrait {
    fn get_name(&self) -> String;
    fn get_ascii_image() -> String;
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum GameMove {
    ROCK,
    PAPER,
    SCISSORS,
    SPOCK,
    LIZARD,
    FIRE,
    WATER,
    QUIT,
}

static NORMAL_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (ROCK, Vec::from([SCISSORS])),
        (PAPER, Vec::from([ROCK])),
        (SCISSORS, Vec::from([PAPER])),
    ])
});

static SPOCK_LIZARD_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (
            ROCK,
            Vec::from([SCISSORS, LIZARD]),
        ),
        (
            PAPER,
            Vec::from([ROCK, SPOCK]),
        ),
        (
            SCISSORS,
            Vec::from([PAPER, LIZARD]),
        ),
        (
            SPOCK,
            Vec::from([ROCK, SCISSORS]),
        ),
        (
            LIZARD,
            Vec::from([PAPER, SPOCK]),
        ),
    ])
});

static FIRE_WATER_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (
            ROCK,
            Vec::from([SCISSORS, FIRE]),
        ),
        (
            PAPER,
            Vec::from([ROCK, WATER]),
        ),
        (
            SCISSORS,
            Vec::from([PAPER]),
        ),
        (
            FIRE,
            Vec::from([PAPER, SCISSORS]),
        ),
        (
            WATER,
            Vec::from([ROCK, SCISSORS, FIRE]),
        ),
    ])
});

impl GameMove {
    pub fn parse_game_move(game_type: &GameType, input: char) -> Result<GameMove, GameError> {
        match input {
            'Q' | 'q' => Ok(QUIT),
            'P' | 'p' => Ok(PAPER),
            'R' | 'r' => Ok(ROCK),
            'S' | 's' => Ok(SCISSORS),
            'O' | 'o' => {
                if *game_type != GameType::SpockLizard {
                    Err(GameError("Wrong character! Please try again.".to_string()))
                } else {
                    Ok(SPOCK)
                }
            }
            'L' | 'l' => {
                if *game_type != GameType::SpockLizard {
                    Err(GameError("Wrong character! Please try again.".to_string()))
                } else {
                    Ok(LIZARD)
                }
            }
            'F' | 'f' => {
                if *game_type != GameType::FireWater {
                    Err(GameError("Wrong character! Please try again.".to_string()))
                } else {
                    Ok(FIRE)
                }
            }
            'W' | 'w' => {
                if *game_type != GameType::FireWater {
                    Err(GameError("Wrong character! Please try again.".to_string()))
                } else {
                    Ok(WATER)
                }
            }
            _ => Err(GameError(
                "Unknown character! Please try again.".to_string(),
            )),
        }
    }

    pub fn beats_other(&self, game_type: &GameType, other: &GameMove) -> GameResult {
        if self == other {
            return GameResult::TIE;
        } else {
            let x = match game_type {
                GameType::Normal => &NORMAL_MATRIX,
                GameType::SpockLizard => &SPOCK_LIZARD_MATRIX,
                GameType::FireWater => &FIRE_WATER_MATRIX,
            };
            if x.get(self).unwrap().contains(other) {
                GameResult::WIN
            } else {
                GameResult::LOSE
            }
        }
    }

    pub fn get_friendly_name(&self) -> String {
        String::from(match self {
            ROCK => "Rock",
            PAPER => "Paper",
            SCISSORS => "Scissors",
            SPOCK => "Spock",
            LIZARD => "Lizard",
            FIRE => "Fire",
            WATER => "Water",
            GameMove::QUIT => "Quit"
        })
    }

    pub fn get_phrase (first: &GameMove, second: &GameMove) -> String {
        String::from(
            if (*first == ROCK && *second == PAPER) || (*second == ROCK && *first == PAPER) {
                "Paper covers rock."
            } else if (*first == ROCK && *second == SCISSORS) || (*second == ROCK && *first == SCISSORS) {
                "Rock crushes scissors."
            } else if (*first == ROCK && *second == LIZARD) || (*second == ROCK && *first == LIZARD) {
                "Rock crushes lizard."
            } else if (*first == ROCK && *second == SPOCK) || (*second == ROCK && *first == SPOCK) {
                "Spock vaporizes rock."
            } else if (*first == ROCK && *second == FIRE) || (*second == ROCK && *first == FIRE) {
                "Rock pounds out fire."
            } else if (*first == ROCK && *second == WATER) || (*second == ROCK && *first == WATER) {
                "Water erodes rock"
            } else if (*first == PAPER && *second == SCISSORS) || (*second == PAPER && *first == SCISSORS) {
                "Scissors cuts paper."
            } else if (*first == PAPER && *second == LIZARD) || (*second == PAPER && *first == LIZARD) {
                "Lizard eats paper."
            } else if (*first == PAPER && *second == SPOCK) || (*second == PAPER && *first == SPOCK) {
                "Paper disproves Spock."
            } else if (*first == PAPER && *second == FIRE) || (*second == PAPER && *first == FIRE) {
                "Fire burns paper."
            } else if (*first == PAPER && *second == WATER) || (*second == PAPER && *first == WATER) {
                "Paper floats on water."
            } else if (*first == SCISSORS && *second == LIZARD) || (*second == SCISSORS && *first == LIZARD) {
                "Scissors decapitates lizard."
            } else if (*first == SCISSORS && *second == SPOCK) || (*second == SCISSORS && *first == SPOCK) {
                "Spock smashes scissors."
            } else if (*first == SCISSORS && *second == FIRE) || (*second == SCISSORS && *first == FIRE) {
                "Fire melts scissors."
            } else if (*first == SCISSORS && *second == WATER) || (*second == SCISSORS && *first == WATER) {
                "Water rusts scissors."
            } else if (*first == LIZARD && *second == SPOCK) || (*second == LIZARD && *first == SPOCK) {
                "Lizard poisons Spock."
            } else if (*first == FIRE && *second == WATER) || (*second == FIRE && *first == WATER) {
                "Water puts out fire."
            } else {
                ""
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_move() {
        let actual_normal_rock = GameMove::parse_game_move(&GameType::Normal, 'R');

        assert!(actual_normal_rock.is_ok());
        assert_eq!(actual_normal_rock.unwrap(), GameMove::ROCK);

        let actual_normal_spock = GameMove::parse_game_move(&GameType::Normal, 'o');
        assert!(actual_normal_spock.is_err());
    }

    #[test]
    fn test_beats_other() {
        assert_eq!(ROCK.beats_other(&GameType::Normal, &PAPER), GameResult::LOSE);
        assert_eq!(ROCK.beats_other(&GameType::Normal, &ROCK), GameResult::TIE);
        assert_eq!(SCISSORS.beats_other(&GameType::Normal, &PAPER), GameResult::WIN);
    }

    #[test]
    fn test_get_phrase() {
        assert_eq!(GameMove::get_phrase(&ROCK, &PAPER), "Paper covers rock.");
        assert_eq!(GameMove::get_phrase(&ROCK, &ROCK), "");
    }

}