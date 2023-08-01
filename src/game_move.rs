use std::collections::HashMap;
use bevy::prelude::*;
use once_cell::sync::Lazy;
use crate::game_move::GameMove::*;
use crate::game_result::GameResult;
use crate::game_type::GameType;

#[derive(Component, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
    Spock,
    Lizard,
    Fire,
    Water,
}

static NORMAL_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (Rock, Vec::from([Scissors])),
        (Paper, Vec::from([Rock])),
        (Scissors, Vec::from([Paper])),
    ])
});

static SPOCK_LIZARD_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (
            Rock,
            Vec::from([Scissors, Lizard]),
        ),
        (
            Paper,
            Vec::from([Rock, Spock]),
        ),
        (
            Scissors,
            Vec::from([Paper, Lizard]),
        ),
        (
            Spock,
            Vec::from([Rock, Scissors]),
        ),
        (
            Lizard,
            Vec::from([Paper, Spock]),
        ),
    ])
});

static FIRE_WATER_MATRIX: Lazy<HashMap<GameMove, Vec<GameMove>>> = Lazy::new(|| {
    HashMap::from([
        (
            Rock,
            Vec::from([Scissors, Fire]),
        ),
        (
            Paper,
            Vec::from([Rock, Water]),
        ),
        (
            Scissors,
            Vec::from([Paper]),
        ),
        (
            Fire,
            Vec::from([Paper, Scissors]),
        ),
        (
            Water,
            Vec::from([Rock, Scissors, Fire]),
        ),
    ])
});

impl GameMove {
    pub fn from_i32(game_type: GameType, value: i32) -> Option<GameMove> {
        match value {
            1 => Some(Rock),
            2 => Some(Paper),
            3 => Some(Scissors),
            _ => {
                if value == 4 {
                    if game_type == GameType::SpockLizard {
                        Some(Spock)
                    } else {
                        Some(Fire)
                    }
                } else if value == 5 {
                    if game_type == GameType::SpockLizard {
                        Some(Lizard)
                    } else {
                        Some(Water)
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn beats_other(&self, game_type: &GameType, other: &GameMove) -> GameResult {
        if self == other {
            return GameResult::Draw;
        } else {
            let x = match game_type {
                GameType::Normal => &NORMAL_MATRIX,
                GameType::SpockLizard => &SPOCK_LIZARD_MATRIX,
                GameType::FireWater => &FIRE_WATER_MATRIX,
                _ => panic!("Incompatible game type. None cannot be used here."),
            };
            if x.get(self).unwrap().contains(other) {
                GameResult::Win
            } else {
                GameResult::Lose
            }
        }
    }

    pub fn get_phrase (first: &GameMove, second: &GameMove) -> String {
        String::from(
            if (*first == Rock && *second == Paper) || (*second == Rock && *first == Paper) {
                "Paper covers Rock."
            } else if (*first == Rock && *second == Scissors) || (*second == Rock && *first == Scissors) {
                "Rock crushes Scissors."
            } else if (*first == Rock && *second == Lizard) || (*second == Rock && *first == Lizard) {
                "Rock crushes Lizard."
            } else if (*first == Rock && *second == Spock) || (*second == Rock && *first == Spock) {
                "Spock vaporizes Rock."
            } else if (*first == Rock && *second == Fire) || (*second == Rock && *first == Fire) {
                "Rock pounds out Fire."
            } else if (*first == Rock && *second == Water) || (*second == Rock && *first == Water) {
                "Water erodes Rock"
            } else if (*first == Paper && *second == Scissors) || (*second == Paper && *first == Scissors) {
                "Scissors cuts Paper."
            } else if (*first == Paper && *second == Lizard) || (*second == Paper && *first == Lizard) {
                "Lizard eats Paper."
            } else if (*first == Paper && *second == Spock) || (*second == Paper && *first == Spock) {
                "Paper disproves Spock."
            } else if (*first == Paper && *second == Fire) || (*second == Paper && *first == Fire) {
                "Fire burns Paper."
            } else if (*first == Paper && *second == Water) || (*second == Paper && *first == Water) {
                "Paper floats on Water."
            } else if (*first == Scissors && *second == Lizard) || (*second == Scissors && *first == Lizard) {
                "Scissors decapitates Lizard."
            } else if (*first == Scissors && *second == Spock) || (*second == Scissors && *first == Spock) {
                "Spock smashes Scissors."
            } else if (*first == Scissors && *second == Fire) || (*second == Scissors && *first == Fire) {
                "Fire melts Scissors."
            } else if (*first == Scissors && *second == Water) || (*second == Scissors && *first == Water) {
                "Water rusts Scissors."
            } else if (*first == Lizard && *second == Spock) || (*second == Lizard && *first == Spock) {
                "Lizard poisons Spock."
            } else if (*first == Fire && *second == Water) || (*second == Fire && *first == Water) {
                "Water puts out Fire."
            } else {
                ""
            }
        )
    }
}