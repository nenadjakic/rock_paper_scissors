use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::common::*;
use crate::game_settings::GameSettings;

#[derive(Component)]
pub struct OnGameOverview;

pub struct GameOverviewPlugin;

impl Plugin for GameOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOverview), setup_score_overview_screen)
            .add_systems(Update, confirm_button_action.run_if(in_state(AppState::GameOverview)))
            .add_systems(OnExit(AppState::GameOverview), despawn_screen::<OnGameOverview>);
    }
}

pub fn setup_score_overview_screen(mut commands: Commands, game_font: Res<GameFont>, game_statistics: Res<GameStatistics>, game_settings: Res<GameSettings>) {
    let font = &game_font.0;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnGameOverview,
        ))
        .with_children(|parent| {
            parent.spawn(
                (TextBundle::from_section(
                    game_settings.player_options.name.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Right))
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(40.0),
                                ..default()
                            },
                            background_color: MENU_BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Game overview",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: OVERVIEW_TITLE_COLOR,
                                        font: font.clone(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(40.0)),
                                    ..default()
                                }),
                            );

                            parent.spawn(
                                TextBundle::from_section(
                                    format!(
                                        "Total: {0}, wins: {1}, loses: {2}, draws: {3}",
                                        game_statistics.totals(),
                                        game_statistics.wins,
                                        game_statistics.loses,
                                        game_statistics.draws
                                    ),
                                    TextStyle {
                                        font_size: 32.0,
                                        color: OVERVIEW_SUB_TITLE_COLOR,
                                        font: font.clone(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(40.0)),
                                    ..default()
                                }),
                            );

                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::End,
                                        justify_content: JustifyContent::End,
                                        align_content: AlignContent::End,
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Px(250.0),
                                                height: Val::Px(50.0),
                                                margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(80.0), Val::Px(20.0)),
                                                border: UiRect::all(Val::Px(5.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            border_color: Color::WHITE.into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(
                                                TextBundle::from_section(
                                                    "(C)ontinue",
                                                    TextStyle {
                                                        font_size: BUTTON_TEXT_SIZE,
                                                        color: BUTTON_TITLE_COLOR,
                                                        font: font.clone(),
                                                    },
                                                )
                                                .with_style(Style {
                                                    margin: UiRect::all(Val::Px(10.0)),
                                                    ..default()
                                                }),
                                            );
                                        });
                                });
                        });
                });
        });
}

pub fn confirm_button_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_statistics: ResMut<GameStatistics>,
    audio: Res<Audio>,
    game_settings: Res<GameSettings>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        game_statistics.reset_scores();
        game_state.set(GameState::NotInit);
        app_state.set(AppState::Menu);

        play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
}
