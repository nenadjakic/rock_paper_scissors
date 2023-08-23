use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::common::*;

#[derive(Component)]
pub struct OnCreditsScreen;

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), setup_credits_screen)
            .add_systems(Update, confirm_button_action.run_if(in_state(AppState::Credits)))
            .add_systems(OnExit(AppState::Credits), despawn_screen::<OnCreditsScreen>);
    }
}

pub fn setup_credits_screen(mut commands: Commands, game_font: Res<GameFont>) {
    let font = &game_font.0;
    let header_style = TextStyle {
        font: font.clone(),
        font_size: 18.0,
        color: OVERVIEW_TITLE_COLOR,
    };

    let body_style = TextStyle {
        font: font.clone(),
        font_size: 14.0,
        color: OVERVIEW_SUB_TITLE_COLOR,
    };

    let header_margin = UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(10.0), Val::Px(10.0));
    let body_margin = UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(5.0), Val::Px(15.0));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnCreditsScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(55.0),
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Credits",
                            TextStyle {
                                font_size: 40.0,
                                color: TITLE_COLOR,
                                font: font.clone(),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(30.0), Val::Px(40.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(TextBundle::from_section("Author", header_style.clone()).with_style(Style {
                        margin: header_margin,
                        ..default()
                    }));

                    parent.spawn(TextBundle::from_section("Nenad Jakic", body_style.clone()).with_style(Style {
                        margin: body_margin,
                        ..default()
                    }));

                    parent.spawn(TextBundle::from_section("Images", header_style.clone()).with_style(Style {
                        margin: header_margin,
                        ..default()
                    }));

                    parent.spawn(
                        TextBundle::from_section(
                            "Downloaded from web site: https://icons8.com, free for personal and commercial use licence.",
                            body_style.clone(),
                        )
                        .with_style(Style {
                            margin: body_margin,
                            ..default()
                        }),
                    );

                    parent.spawn(TextBundle::from_section("Sounds", header_style.clone()).with_style(Style {
                        margin: header_margin,
                        ..default()
                    }));

                    parent.spawn(
                        TextBundle::from_section("Downloaded from web site: https://pixabay.com/, free for use licence.", body_style.clone()).with_style(Style {
                            margin: body_margin,
                            ..default()
                        }),
                    );

                    parent.spawn(TextBundle::from_section("Fonts", header_style.clone()).with_style(Style {
                        margin: header_margin,
                        ..default()
                    }));

                    parent.spawn(
                        TextBundle::from_section("Downloaded from web site: https://fonts.google.com/, licensed under Open Font Licence.", body_style.clone()).with_style(Style {
                            margin: body_margin,
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
                                margin: UiRect::top(Val::Px(20.0)),
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
                                        margin: UiRect::all(Val::Px(10.0)),
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
                                            "(B)ack",
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
}

pub fn confirm_button_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    audio: Res<Audio>,
    settings: Res<GameSettings>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.just_pressed(KeyCode::B) {
        app_state.set(AppState::Menu);

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}
