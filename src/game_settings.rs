use crate::player_options::PlayerOptions;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const GAME_SETTINGS_FILE_PATH: &str = "./GAME_SETTINGS";
#[derive(Resource, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSettings {
    pub player_options: PlayerOptions,
    pub is_sound_on: bool,
}

impl GameSettings {
    pub fn init() -> Self {
        let file_path = Path::new(GAME_SETTINGS_FILE_PATH);
        if file_path.exists() {
            let mut content = String::new();
            let mut file = File::open(GAME_SETTINGS_FILE_PATH).unwrap_or_else(|_| panic!("Unable to open file: {}", GAME_SETTINGS_FILE_PATH));
            file.read_to_string(&mut content)
                .unwrap_or_else(|_| panic!("Unable to read from file: {}", GAME_SETTINGS_FILE_PATH));
            if let Ok(x) = serde_json::from_str(&content) {
                return x;
            }
        }
        GameSettings::new_and_persist()
    }

    pub fn fetch(&self) {
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(Path::new(GAME_SETTINGS_FILE_PATH))
            .unwrap_or_else(|_| panic!("Unable to open/create file: {}", GAME_SETTINGS_FILE_PATH));

        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .unwrap_or_else(|_| panic!("Unable to write to file: {}", GAME_SETTINGS_FILE_PATH));
    }

    fn new() -> Self {
        Self {
            player_options: PlayerOptions::new(),
            is_sound_on: true,
        }
    }
    fn new_and_persist() -> Self {
        println!("DDDDDDD");
        let game_settings = GameSettings::new();
        let mut file = File::create(GAME_SETTINGS_FILE_PATH).unwrap_or_else(|_| panic!("Unable to create file: {}", GAME_SETTINGS_FILE_PATH));
        file.write_all(serde_json::to_string(&game_settings).unwrap().as_bytes())
            .unwrap_or_else(|_| panic!("Unable to write to file: {}", GAME_SETTINGS_FILE_PATH));
        game_settings
    }
}
