use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use crate::game_move::GameMove;
use crate::game_result::GameResult;

pub const TITLE_TEXT: &str = "rock paper scissors";
pub const TITLE_COLOR: Color = Color::rgb(0.16471, 0.61569, 0.56078);
pub const TITLE_SIZE: f32 = 32.0;

pub const MENU_BACKGROUND_COLOR: Color = Color::rgb(0.91373, 0.76863, 0.41569);

pub const BUTTON_TITLE_COLOR: Color = Color::rgb(0.58039, 0.10980, 0.18431);
pub const BUTTON_TEXT_SIZE: f32 = 24.0;
pub const BUTTON_TEXT_SMALL_SIZE: f32 = 16.0;

pub const GAME_SELECTED_BORDER_COLOR: Color = Color::RED;
pub const GAME_NO_SELECTED_BORDER_COLOR: Color = Color::ANTIQUE_WHITE;

pub const OVERVIEW_BACKGROUND_COLOR: Color = Color::rgb(0.45098, 0.30980, 0.35294);
pub const OVERVIEW_TITLE_COLOR: Color = Color::rgb(0.90588,0.43529, 0.31765);
pub const OVERVIEW_SUB_TITLE_COLOR: Color = Color::rgb(0.90588, 0.53529, 0.35039);

pub const CLOSING_DURATION: f32 = 2.0;

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct GameSettings {
    pub is_sound_on: bool
}
impl GameSettings {
    pub fn init() -> Self {
        Self {
            is_sound_on: true,
        }
    }
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct GameStatistics {
    pub last_round_result: Option<GameResult>,
    pub last_player_move: Option<GameMove>,
    pub last_computer_move: Option<GameMove>,
    pub wins: u32,
    pub loses: u32,
    pub draws: u32
}
impl GameStatistics {
    pub fn init() -> Self {
        Self {
            last_round_result: None,
            last_player_move: None,
            last_computer_move: None,
            wins: 0,
            loses: 0,
            draws: 0,
        }
    }

    pub fn reset_scores(&mut self) {
        self.last_round_result = None;
        self.last_player_move = None;
        self.last_computer_move = None;
        self.wins = 0;
        self.loses = 0;
        self.draws = 0;
    }

    pub fn totals(&self) -> u32 {
        self.wins + self.loses + self.draws
    }
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct SelectedOption {
    pub value: i32
}
impl SelectedOption {
    pub fn init() -> Self {
        Self {
            value: 1
        }
    }

    pub fn set_value(&mut self, v: i32) {
        self.value = v;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    //Splash
    #[default]
    Menu,
    Playing,
    GameOverview,
    Credits,
    Closing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum MenuState {
    #[default]
    NotInit,
    StartMenu,
    SettingsMenu,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    NotInit,
    PlayerMoveRender,
    PlayerMove,
    RoundFinish,
}

#[derive(Debug, Resource)]
pub struct GameSounds {
    pub mode_switch: Handle<bevy_kira_audio::AudioSource>,
    pub win: Handle<bevy_kira_audio::AudioSource>,
    pub lose: Handle<bevy_kira_audio::AudioSource>,
    pub drawn: Handle<bevy_kira_audio::AudioSource>,
}

pub fn setup_game_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource( GameSounds {
        mode_switch: asset_server.load("sounds/light-switch-156813.ogg"),
        win: asset_server.load("sounds/tada-fanfare-a-6313.ogg"),
        lose: asset_server.load("sounds/negative_beeps-6008.ogg"),
        drawn: asset_server.load("sounds/light-switch-156813.ogg"),
    });
}

pub fn play_sound(audio: &Res<Audio>, is_sound_on: bool, sound: &Handle<bevy_kira_audio::AudioSource>) {
    if is_sound_on {
        audio.play(sound.clone());
    }
}

#[derive(Debug, Resource)]
pub struct GameImages {
    pub joystick: Handle<Image>,
    pub rock: Handle<Image>,
    pub paper: Handle<Image>,
    pub scissors: Handle<Image>,
    pub spock: Handle<Image>,
    pub lizard: Handle<Image>,
    pub fire: Handle<Image>,
    pub water: Handle<Image>,
}

pub fn setup_game_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameImages {
        joystick: asset_server.load("images/joystick-50.png"),
        rock: asset_server.load("images/rock-100.png"),
        paper: asset_server.load("images/paper-100.png"),
        scissors: asset_server.load("images/scissors-100.png"),
        spock: asset_server.load("images/spock-100.png"),
        lizard: asset_server.load("images/lizard-100.png"),
        fire: asset_server.load("images/fire-100.png"),
        water: asset_server.load("images/water-100.png"),
    });
}

#[derive(Debug, Resource)]
pub struct GameFont(pub Handle<Font>);

pub fn setup_game_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameFont(asset_server.load("fonts/PressStart2P-Regular.ttf")));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
