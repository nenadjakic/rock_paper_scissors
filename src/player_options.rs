use std::string::ToString;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerOptions {
    pub uuid: Uuid,
    pub name: String,
}

impl PlayerOptions {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Self {
            uuid,
            name: String::from("PLAYER-") + &*uuid.to_string(),
        }
    }
}
