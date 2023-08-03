use bevy::prelude::*;
use bevy_kira_audio::Audio;
use crate::common::*;

#[derive(Component)]
pub struct OnGameOverview;

pub struct GameOverviewPlugin;

impl Plugin for GameOverviewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOverview), setup_score_overview_screen)
            .add_systems(Update, confirm_button_action.run_if(in_state(AppState::GameOverview)))
            .add_systems(OnExit(AppState::GameOverview), despawn_screen::<OnGameOverview>);
    }
}

pub fn setup_score_overview_screen(mut commands: Commands, game_font: Res<GameFont>, statistics: Res<GameStatistics>
) {
    let font = &game_font.0;

    commands.spawn((NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }, OnGameOverview))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(40.0),
                    ..default()
                },
                background_color: MENU_BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Game overview",
                        TextStyle {
                            font_size: 40.0,
                            color: OVERVIEW_TITLE_COLOR,
                            font: font.clone(),
                        },
                    ).with_style(Style {
                        margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(40.0)),
                        ..default()
                    }),
                );

                parent.spawn(
                    TextBundle::from_section(
                        format!("Total: {0}, wins: {1}, loses: {2}, draws: {3}", statistics.totals(), statistics.wins, statistics.loses, statistics.draws),
                        TextStyle {
                            font_size: 32.0,
                            color: OVERVIEW_SUB_TITLE_COLOR,
                            font: font.clone(),
                        },
                    ).with_style(Style {
                        margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(40.0)),
                        ..default()
                    }),
                );

                parent.spawn(
                    (NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::End,
                            justify_content: JustifyContent::End,
                            align_content: AlignContent::End,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },)).with_children(|parent| {
                    parent.spawn(
                        NodeBundle {
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
                            parent.spawn(TextBundle::from_section(
                                "(C)ontinue",
                                TextStyle {
                                    font_size: BUTTON_TEXT_SIZE,
                                    color: BUTTON_TITLE_COLOR,
                                    font: font.clone(),
                                },
                            ).with_style(Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            }));
                        });
                });
            });
        });
}

pub fn confirm_button_action(keyboard_input: Res<Input<KeyCode>>,
                             mut app_state: ResMut<NextState<AppState>>,
                             mut game_state: ResMut<NextState<GameState>>,
                             mut statistics: ResMut<GameStatistics>,
                             audio: Res<Audio>, settings: Res<GameSettings>, game_sounds: Res<GameSounds>
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        statistics.reset_scores();
        game_state.set(GameState::NotInit);
        app_state.set(AppState::Menu);

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}