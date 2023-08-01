use bevy::app::App;
use bevy::prelude::*;
use bevy::ui::Style;
use bevy_kira_audio::{Audio};
use rand::Rng;
use crate::common::*;
use crate::game_move::GameMove;
use crate::game_result::GameResult;
use crate::game_type::GameType;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OnRoundOverview;

#[derive(Component)]
pub struct OnGamePanel;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(OnEnter(AppState::Playing), init_game_setup)
            .add_systems(OnEnter(GameState::PlayerMoveRender), setup_game_screen)
            .add_systems(OnEnter(GameState::PlayerMove), setup_player_move_screen)
            .add_systems(Update, (switch_game_move, confirm_button_action, confirm_game_move).run_if(in_state(GameState::PlayerMove)))
            .add_systems(Update, confirm_sub_button_action.run_if(in_state(GameState::RoundFinish)))
            .add_systems(OnEnter(GameState::RoundFinish), setup_round_overview_screen)
            .add_systems(OnExit(GameState::RoundFinish), despawn_screen::<OnRoundOverview>)
            .add_systems(OnExit(AppState::Playing), despawn_screen::<OnGameScreen>);
    }
}

pub fn init_game_setup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::PlayerMoveRender)
}

pub fn setup_game_screen(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>,
                         game_font: Res<GameFont>, game_images: Res<GameImages>, game_type: Res<GameType>) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(250.0),
        margin: UiRect::all(Val::Px(10.0)),
        border: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(150.0),
        ..default()
    };

    let font = &game_font.0;

    commands.spawn((NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }, OnGameScreen))
        .with_children(|parent| {
            parent.spawn(
                (NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                },
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            game_type.get_friendly_title(),
                            TextStyle {
                                font_size: TITLE_SIZE,
                                color: TITLE_COLOR,
                                font: font.clone(),
                            },
                        ).with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    parent.spawn((NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    }, OnGamePanel)).with_children(|parent| {
                        spawn_game_move_button(parent, &button_style, true, GameMove::Rock, &button_icon_style, &game_images.rock);
                        spawn_game_move_button(parent, &button_style, false, GameMove::Paper, &button_icon_style, &game_images.paper);
                        spawn_game_move_button(parent, &button_style, false, GameMove::Scissors, &button_icon_style, &game_images.scissors);

                        if *game_type == GameType::SpockLizard {
                            spawn_game_move_button(parent, &button_style, false, GameMove::Spock, &button_icon_style, &game_images.spock);
                            spawn_game_move_button(parent, &button_style, false, GameMove::Lizard, &button_icon_style, &game_images.lizard);
                        } else if *game_type == GameType::FireWater {
                            spawn_game_move_button(parent, &button_style, false, GameMove::Fire, &button_icon_style, &game_images.fire);
                            spawn_game_move_button(parent, &button_style, false, GameMove::Water, &button_icon_style, &game_images.water);
                        }
                    });

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
                        }, OnGamePanel)).with_children(|parent| {
                        parent.spawn(
                            NodeBundle {
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
                                parent.spawn(TextBundle::from_section(
                                    "(F)inish",
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

    game_state.set(GameState::PlayerMove);
}

pub fn setup_player_move_screen(mut query_1: Query<&mut Visibility, With<OnGamePanel>>,
                                mut query_2: Query<(&mut BorderColor, &GameMove), With<GameMove>>,
                                selected_option: Res<SelectedOption>,) {
    debug!("setup_player_move_screen");

        for (mut border_color, game_move) in &mut query_2 {
            match game_move {
                GameMove::Rock => {
                    if selected_option.value == 1 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }

                GameMove::Paper => {
                    if selected_option.value == 2 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }

                GameMove::Scissors => {
                    if selected_option.value == 3 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Spock => {
                    if selected_option.value == 4 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Lizard => {
                    if selected_option.value == 5 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Fire => {
                    if selected_option.value == 4 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Water => {
                    if selected_option.value == 5 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
            }
    }

    for mut visibility in &mut query_1 {
        *visibility = Visibility::Visible;
    }
}

pub fn setup_round_overview_screen(mut commands: Commands, game_font: Res<GameFont>, statistics: Res<GameStatistics>) {
    let font= &game_font.0;

    commands.spawn((NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(90.0),
            height: Val::Percent(35.0),
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(5.0),
            ..default()
        },
        background_color: OVERVIEW_BACKGROUND_COLOR.into(),
        ..default()
    }, OnRoundOverview))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(statistics.last_round_result.expect("Last result is not set.").get_friendly_name(), TextStyle {
                font: font.clone(),
                font_size: 48.0,
                color: OVERVIEW_TITLE_COLOR.into(),
            }).with_style(Style {
                margin: UiRect::new(default(), default(), Val::Px(20.0), Val::Px(20.0)),
                ..default()
            }));

            parent.spawn(TextBundle::from_section(GameMove::get_phrase(&statistics.last_player_move.expect("Last player move is not set."), &statistics.last_computer_move.expect("Last computer move is not set.")),
                                                  TextStyle {
                                                      font: font.clone(),
                                                      font_size: 24.0,
                                                      color: OVERVIEW_SUB_TITLE_COLOR.into(),
                                                  }).with_style(Style {
                margin: UiRect::new(default(), default(), Val::Px(20.0), Val::Px(20.0)),
                ..default()
            }));

            parent.spawn(TextBundle::from_section(format!("Wins: {0}, Loses: {1}, Draws: {2}", statistics.wins, statistics.loses, statistics.draws),
                                                  TextStyle {
                                                      font: font.clone(),
                                                      font_size: 16.0,
                                                      color: OVERVIEW_SUB_TITLE_COLOR.into(),
                                                  }).with_style(Style {
                margin: UiRect::new(default(), default(), Val::Px(35.0), Val::Px(30.0)),
                ..default()
            }));

            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::End,
                        align_content: AlignContent::End,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                parent.spawn(
                    NodeBundle {
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
                        parent.spawn(TextBundle::from_section(
                            "(C)ontinue",
                            TextStyle {
                                font_size: BUTTON_TEXT_SMALL_SIZE,
                                color: Color::WHITE,
                                font: font.clone(),
                            },
                        ).with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }));
                    });

                parent.spawn(
                    NodeBundle {
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
                        parent.spawn(TextBundle::from_section(
                            "(F)inish",
                            TextStyle {
                                font_size: BUTTON_TEXT_SMALL_SIZE,
                                color: Color::WHITE,
                                font: font.clone(),
                            },
                        ).with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }));
                    });
            });
        });
}

fn spawn_game_move_button(parent: &mut ChildBuilder, style: &Style, bordered: bool, game_move: GameMove,
                          icon_style: &Style, image: &Handle<Image>,
) {
    parent.spawn(
        (NodeBundle {
            style: style.clone(),
            border_color: if bordered { GAME_SELECTED_BORDER_COLOR.into() } else { GAME_NO_SELECTED_BORDER_COLOR.into() },
            ..default()
        }, game_move))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: icon_style.clone(),
                image: UiImage::new(image.clone()),
                ..default()
            });
        });
}

pub fn confirm_sub_button_action(keyboard_input: Res<Input<KeyCode>>,
                                 mut selected_option: ResMut<SelectedOption>,
                                 mut game_state: ResMut<NextState<GameState>>,
                                 mut app_state: ResMut<NextState<AppState>>,
                                 audio: Res<Audio>,
                                 game_sounds: Res<GameSounds>,
                                 settings: Res<GameSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        selected_option.set_value(1);
        game_state.set(GameState::PlayerMove);

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    } else if keyboard_input.just_pressed(KeyCode::F) {
        selected_option.set_value(1);
        game_state.set(GameState::NotInit);
        app_state.set(AppState::GameOverview);

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_button_action(keyboard_input: Res<Input<KeyCode>>,
                             mut selected_option: ResMut<SelectedOption>,
                             mut app_state: ResMut<NextState<AppState>>,
                             mut game_state: ResMut<NextState<GameState>>,
                             mut menu_state: ResMut<NextState<MenuState>>,
                             audio: Res<Audio>,
                             game_sounds: Res<GameSounds>,
                             settings: ResMut<GameSettings>,
                             statistics: Res<GameStatistics>
) {
    if keyboard_input.just_pressed(KeyCode::F) {
        selected_option.set_value(1);
        if statistics.last_round_result.is_none() {
            game_state.set(GameState::NotInit);
            menu_state.set(MenuState::StartMenu);
            app_state.set(AppState::Menu);
        } else {
            app_state.set(AppState::GameOverview);
        }
        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn switch_game_move(keyboard_input: Res<Input<KeyCode>>,
                        mut query: Query<(&mut BorderColor, &GameMove), With<GameMove>>,
                        mut selected_option: ResMut<SelectedOption>,
                        audio: Res<Audio>,
                        game_sounds: Res<GameSounds>,
                        settings: Res<GameSettings>,
                        game_type: Res<GameType>
) {
    let mut left_or_down: bool = false;
    if keyboard_input.just_pressed(KeyCode::Left) {
        left_or_down = true;

        if selected_option.value > 1 {
            selected_option.value -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        left_or_down = true;

        if selected_option.value < game_type.max_number_of_move() {
            selected_option.value += 1;
        }
    }

    if left_or_down {
        for (mut border_color, game_move) in &mut query {
            match game_move {
                GameMove::Rock => {
                    if selected_option.value == 1 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }

                GameMove::Paper => {
                    if selected_option.value == 2 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }

                GameMove::Scissors => {
                    if selected_option.value == 3 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Spock => {
                    if selected_option.value == 4 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Lizard => {
                    if selected_option.value == 5 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Fire => {
                    if selected_option.value == 4 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
                GameMove::Water => {
                    if selected_option.value == 5 {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    }
                }
            }
        }
        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_game_move(keyboard_input: Res<Input<KeyCode>>,
                         mut query: Query<&mut Visibility, With<OnGamePanel>>,
                         selected_option: ResMut<SelectedOption>,
                         audio: Res<Audio>,
                         game_sounds: Res<GameSounds>,
                         game_type: Res<GameType>,
                         mut statistics: ResMut<GameStatistics>,
                         mut game_state: ResMut<NextState<GameState>>,
                         settings: Res<GameSettings>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        let player_move: Option<GameMove> = GameMove::from_i32(*game_type, selected_option.get_value());
        match player_move {
            Some(x) => {
                let computer_move: GameMove = get_computer_move(&game_type);
                let game_result = x.beats_other(&game_type, &computer_move);

                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }

                match game_result {
                    GameResult::Win => {
                        statistics.wins += 1;
                    }
                    GameResult::Lose => {
                        statistics.loses += 1;
                    }
                    GameResult::Draw => {
                        statistics.draws += 1;
                    }
                }
                statistics.last_round_result = Some(game_result.clone());
                statistics.last_computer_move = Some(computer_move);
                statistics.last_player_move = Some(x);
                debug!("computer: {:?}", statistics.last_computer_move);
                debug!("player: {:?}", statistics.last_player_move);

                game_state.set(GameState::RoundFinish);
            }
            _ => {}
        }
        let result_sound = match statistics.last_round_result.unwrap() {
            GameResult::Win => &game_sounds.win,
            GameResult::Lose => &game_sounds.lose,
            GameResult::Draw => &game_sounds.drawn,
        };
        play_sound(&audio, settings.is_sound_on, result_sound);
    }
}

fn get_computer_move(game_type: &GameType) -> GameMove {
    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(1..=game_type.max_number_of_move());
    match random_value {
        1 => GameMove::Rock,
        2 => GameMove::Paper,
        3 => GameMove::Scissors,
        4 => {
            if *game_type == GameType::SpockLizard {
                GameMove::Spock
            } else {
                GameMove::Fire
            }
        }
        _ => {
            if *game_type == GameType::SpockLizard {
                GameMove::Lizard
            } else {
                GameMove::Water
            }
        }
    }
}