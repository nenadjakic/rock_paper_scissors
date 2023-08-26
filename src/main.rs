use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_kira_audio::AudioPlugin;

use crate::closing::ClosingPlugin;
use crate::common::*;
use crate::credits::CreditsPlugin;
use crate::game::GamePlugin;
use crate::game_overview::GameOverviewPlugin;
use crate::game_settings::GameSettings;
use crate::game_type::GameType;
use crate::menu::MenuPlugin;

mod closing;
mod common;
mod credits;
mod game;
mod game_move;
mod game_overview;
mod game_result;
mod game_settings;
mod game_type;
mod menu;
mod player_options;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wgpu=warn,bevy_ecs=info,rock_paper_scissors=debug".to_string(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rock paper scissors".to_string(),
                        mode: WindowMode::Fullscreen,
                        ..default()
                    }),
                    ..default()
                }),
            AudioPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameType::None)
        .insert_resource(SelectedOption::init())
        .insert_resource(GameSettings::init())
        .insert_resource(GameStatistics::init())
        .add_systems(
            Startup,
            (setup_camera, setup_game_sounds, setup_game_images, setup_game_font, setup_game_settings),
        )
        .add_plugins((MenuPlugin, ClosingPlugin, GamePlugin, GameOverviewPlugin, CreditsPlugin))
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
