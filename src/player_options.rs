use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::string::ToString;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PLAYER_OPTIONS_FILE_PATH: &str = "./PLAYER_OPTIONS";

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct PlayerOptions {
    pub uuid: Uuid,
    pub name: String,
}

impl PlayerOptions {
    pub fn new() -> Self {
        let file_path = Path::new(PLAYER_OPTIONS_FILE_PATH);
        if file_path.exists() {
            let mut content = String::new();
            let mut file = File::open(PLAYER_OPTIONS_FILE_PATH).unwrap_or_else(|_| panic!("Unable to open file: {}", PLAYER_OPTIONS_FILE_PATH));
            file.read_to_string(&mut content).unwrap_or_else(|_| panic!("Unable to read from file: {}", PLAYER_OPTIONS_FILE_PATH));
            match serde_json::from_str(&content) {
                Ok(x) => x,
                Err(_) => create_player_options(),
            }
        } else {
            create_player_options()
        }
    }

    pub fn push(&self) {
        let file_path = Path::new(PLAYER_OPTIONS_FILE_PATH);
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap_or_else(|_| panic!("Unable to open/create file: {}", PLAYER_OPTIONS_FILE_PATH));

        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .unwrap_or_else(|_| panic!("Unable to write to file: {}", PLAYER_OPTIONS_FILE_PATH));
    }
}

fn create_player_options() -> PlayerOptions {
    let uuid = Uuid::new_v4();
    let player_options = PlayerOptions {
        uuid,
        name: String::from("PLAYER-") + &*uuid.to_string(),
    };

    let mut file = File::create(PLAYER_OPTIONS_FILE_PATH).unwrap_or_else(|_| panic!("Unable to create file: {}", PLAYER_OPTIONS_FILE_PATH));
    file.write_all(serde_json::to_string(&player_options).unwrap().as_bytes())
        .unwrap_or_else(|_| panic!("Unable to write to file: {}", PLAYER_OPTIONS_FILE_PATH));
    player_options
}