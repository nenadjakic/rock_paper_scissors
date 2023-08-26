use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::cmp::min;

use crate::common::*;
use crate::game_settings::GameSettings;
use crate::game_type::GameType;

#[derive(Component)]
pub struct OnStartMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct OnChangeNameScreen;

#[derive(Component)]
pub struct OnChangeName;

#[derive(Component, Debug)]
pub enum SaveCancelAction {
    Save,
    Cancel,
}

#[derive(Resource, Deref, DerefMut)]
pub struct BlinkingTimer(Timer);

#[derive(Component)]
pub struct OnGameSound;

#[derive(Component)]
pub enum MenuAction {
    Normal,
    SpockLizard,
    FireWater,
    Settings,
    Credits,
    Exit,
}

#[derive(Component)]
pub enum SettingAction {
    Sound,
    ChangeName,
    Back,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_event::<OnKeyPressEvent>()
            .add_systems(OnEnter(AppState::Menu), init_setup_menu)
            .add_systems(OnEnter(MenuState::StartMenu), setup_start_menu)
            .add_systems(
                Update,
                (switch_start_menu_action, confirm_start_menu_action).run_if(in_state(MenuState::StartMenu)),
            )
            .add_systems(OnExit(MenuState::StartMenu), despawn_screen::<OnStartMenuScreen>)
            .add_systems(OnEnter(MenuState::SettingsMenu), setup_setting_menu)
            .add_systems(
                Update,
                (switch_settings_menu_action, confirm_settings_menu_action).run_if(in_state(MenuState::SettingsMenu)),
            )
            .add_systems(OnExit(MenuState::SettingsMenu), despawn_screen::<OnSettingsMenuScreen>)
            .add_systems(OnEnter(MenuState::ChangeName), setup_change_name_screen)
            .add_systems(
                Update,
                (on_key_press_event_trigger, on_key_press_event_listener, on_blinking_text_indicator).run_if(in_state(MenuState::ChangeName)),
            )
            .add_systems(OnExit(MenuState::ChangeName), despawn_screen::<OnChangeNameScreen>);
    }
}

pub fn init_setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::StartMenu);
}

pub fn setup_start_menu(mut commands: Commands, game_font: Res<GameFont>, game_images: Res<GameImages>, game_settings: Res<GameSettings>) {
    let button_style = Style {
        flex_direction: FlexDirection::Row,
        width: Val::Px(400.0),
        height: Val::Px(40.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Start,
        justify_items: JustifyItems::Start,
        align_items: AlignItems::Start,
        ..default()
    };

    let button_icon_style = Style {
        width: Val::Px(24.0),
        height: Val::Px(24.0),
        ..default()
    };

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
            OnStartMenuScreen,
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
                                ..default()
                            },
                            background_color: MENU_BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Display the game name
                            parent.spawn(
                                TextBundle::from_section(
                                    TITLE_TEXT,
                                    TextStyle {
                                        font_size: TITLE_SIZE,
                                        color: TITLE_COLOR,
                                        font: font.clone(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                }),
                            );
                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                visibility: Visibility::Visible,
                                text: "Normal",
                                button_style: &button_style,
                                icon_style: &button_icon_style,
                                icon: &game_images.joystick,
                                menu_action: MenuAction::Normal,
                                font,
                                }
                            );

                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                    visibility: Visibility::Hidden,
                                    text: "Spock lizard",
                                    button_style: &button_style,
                                    icon_style: &button_icon_style,
                                    icon: &game_images.joystick,
                                    menu_action: MenuAction::SpockLizard,
                                    font,
                                }
                            );

                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                    visibility: Visibility::Hidden,
                                    text: "Fire water",
                                    button_style: &button_style,
                                    icon_style: &button_icon_style,
                                    icon: &game_images.joystick,
                                    menu_action: MenuAction::FireWater,
                                    font,
                                }
                            );

                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                visibility: Visibility::Hidden,
                                    text: "Settings",
                                    button_style: &button_style,
                                    icon_style: &button_icon_style,
                                    icon: &game_images.joystick,
                                    menu_action: MenuAction::Settings,
                                    font,
                                }
                            );

                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                    visibility: Visibility::Hidden,
                                    text: "Credits",
                                    button_style: &button_style,
                                    icon_style: &button_icon_style,
                                    icon: &game_images.joystick,
                                    menu_action: MenuAction::Credits,
                                    font,
                                }
                            );

                            spawn_start_menu_button(
                                parent,
                                StartMenuButtonOptions {
                                    visibility: Visibility::Hidden,
                                    text: "Exit",
                                    button_style: &button_style,
                                    icon_style: &button_icon_style,
                                    icon: &game_images.joystick,
                                    menu_action: MenuAction::Exit,
                                    font,
                                }
                            );
                        });
                });
        });
}

pub fn setup_setting_menu(mut commands: Commands, game_font: Res<GameFont>, game_images: Res<GameImages>, game_settings: Res<GameSettings>) {
    let button_style = Style {
        flex_direction: FlexDirection::Row,
        width: Val::Px(300.0),
        height: Val::Px(40.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Start,
        justify_items: JustifyItems::Start,
        align_items: AlignItems::Start,
        ..default()
    };

    let button_icon_style = Style {
        width: Val::Px(24.0),
        height: Val::Px(24.0),
        ..default()
    };

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
            OnSettingsMenuScreen,
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
                                ..default()
                            },
                            background_color: MENU_BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Settings",
                                    TextStyle {
                                        font_size: TITLE_SIZE,
                                        color: TITLE_COLOR,
                                        font: font.clone(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                }),
                            );

                            parent
                                .spawn(NodeBundle {
                                    style: button_style.clone(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        ImageBundle {
                                            style: button_icon_style.clone(),
                                            image: UiImage::new(game_images.joystick.clone()),
                                            visibility: Visibility::Visible,
                                            ..default()
                                        },
                                        SettingAction::Sound,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Sound (".to_string() + if game_settings.is_sound_on { "On" } else { "Off" } + ")",
                                            TextStyle {
                                                font_size: BUTTON_TEXT_SIZE,
                                                color: BUTTON_TITLE_COLOR,
                                                font: font.clone(),
                                            },
                                        )
                                        .with_style(Style {
                                            margin: UiRect::left(Val::Px(10.0)),
                                            ..default()
                                        }),
                                        OnGameSound,
                                    ));
                                });

                            parent
                                .spawn(NodeBundle {
                                    style: button_style.clone(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        ImageBundle {
                                            style: button_icon_style.clone(),
                                            image: UiImage::new(game_images.joystick.clone()),
                                            visibility: Visibility::Hidden,
                                            ..default()
                                        },
                                        SettingAction::ChangeName,
                                    ));

                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Change name",
                                            TextStyle {
                                                font_size: BUTTON_TEXT_SIZE,
                                                color: BUTTON_TITLE_COLOR,
                                                font: font.clone(),
                                            },
                                        )
                                        .with_style(Style {
                                            margin: UiRect::left(Val::Px(10.0)),
                                            ..default()
                                        }),
                                    );
                                });

                            parent
                                .spawn(NodeBundle {
                                    style: button_style.clone(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        ImageBundle {
                                            style: button_icon_style.clone(),
                                            image: UiImage::new(game_images.joystick.clone()),
                                            visibility: Visibility::Hidden,
                                            ..default()
                                        },
                                        SettingAction::Back,
                                    ));
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Back",
                                            TextStyle {
                                                font_size: BUTTON_TEXT_SIZE,
                                                color: BUTTON_TITLE_COLOR,
                                                font: font.clone(),
                                            },
                                        )
                                        .with_style(Style {
                                            margin: UiRect::left(Val::Px(10.0)),
                                            ..default()
                                        }),
                                    );
                                });
                        });
                });
        });
}

pub fn setup_change_name_screen(mut commands: Commands, game_font: Res<GameFont>, game_settings: Res<GameSettings>) {
    let button_style = Style {
        flex_direction: FlexDirection::Row,
        width: Val::Percent(90.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(10.0)),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Start,
        justify_items: JustifyItems::Start,
        align_items: AlignItems::Start,
        ..default()
    };

    let font = &game_font.0;

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
            OnChangeNameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Change name",
                            TextStyle {
                                font_size: TITLE_SIZE,
                                color: TITLE_COLOR,
                                font: font.clone(),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn(NodeBundle {
                            style: button_style.clone(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    game_settings.player_options.name.clone(),
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
                                OnChangeName,
                            ));
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::RowReverse,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(250.0),
                                            height: Val::Px(50.0),
                                            margin: UiRect::new(Val::Px(20.0), Val::Px(40.0), Val::Px(40.0), Val::Px(20.0)),
                                            border: UiRect::all(Val::Px(7.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: GAME_NO_SELECTED_BORDER_COLOR.into(),
                                        ..default()
                                    },
                                    SaveCancelAction::Cancel,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Cancel",
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
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(250.0),
                                            height: Val::Px(50.0),
                                            margin: UiRect::new(Val::Px(20.0), Val::Px(40.0), Val::Px(40.0), Val::Px(20.0)),
                                            border: UiRect::all(Val::Px(7.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: GAME_SELECTED_BORDER_COLOR.into(),
                                        ..default()
                                    },
                                    SaveCancelAction::Save,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Save",
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

    commands.insert_resource(BlinkingTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

struct StartMenuButtonOptions<'a> {
    visibility: Visibility,
    text: &'a str,
    button_style: &'a Style,
    icon_style: &'a Style,
    icon: &'a Handle<Image>,
    menu_action: MenuAction,
    font: &'a Handle<Font>,
}

fn spawn_start_menu_button(
    parent: &mut ChildBuilder,
    options: StartMenuButtonOptions
) {
    parent
        .spawn(NodeBundle {
            style: options.button_style.clone(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: options.icon_style.clone(),
                    image: UiImage::new(options.icon.clone()),
                    visibility: options.visibility,
                    ..default()
                },
                options.menu_action,
            ));
            parent.spawn(
                TextBundle::from_section(
                    options.text,
                    TextStyle {
                        font_size: BUTTON_TEXT_SIZE,
                        color: BUTTON_TITLE_COLOR,
                        font: options.font.clone(),
                    },
                )
                .with_style(Style {
                    margin: UiRect::left(Val::Px(10.0)),
                    ..default()
                }),
            );
        });
}

pub fn switch_start_menu_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_type: ResMut<GameType>,
    mut query: Query<(&mut Visibility, &MenuAction), With<MenuAction>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
    mut selected_option: ResMut<SelectedOption>,
    game_settings: Res<GameSettings>,
) {
    let mut up_or_down = false;
    if keyboard_input.just_pressed(KeyCode::Up) {
        up_or_down = true;

        if selected_option.value > 1 {
            selected_option.value -= 1;
        }

        *game_type = selected_option.value.into();
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        up_or_down = true;

        if selected_option.value < 6 {
            selected_option.value += 1;
        }

        *game_type = selected_option.value.into();
    }
    if up_or_down {
        for (mut visibility, menu_action) in &mut query {
            match menu_action {
                MenuAction::Normal => {
                    if *game_type == GameType::Normal {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                MenuAction::SpockLizard => {
                    if *game_type == GameType::SpockLizard {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                MenuAction::FireWater => {
                    if *game_type == GameType::FireWater {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                MenuAction::Settings => {
                    if selected_option.value == 4 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                MenuAction::Credits => {
                    if selected_option.value == 5 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                MenuAction::Exit => {
                    if selected_option.value == 6 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }

        play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn switch_settings_menu_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Visibility, &SettingAction), With<SettingAction>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
    mut selected_option: ResMut<SelectedOption>,
    game_settings: Res<GameSettings>,
) {
    let mut up_or_down = false;
    if keyboard_input.just_pressed(KeyCode::Up) {
        up_or_down = true;

        if selected_option.value > 1 {
            selected_option.value -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        up_or_down = true;

        if selected_option.value < 3 {
            selected_option.value += 1;
        }
    }
    if up_or_down {
        for (mut visibility, setting_action) in &mut query {
            match setting_action {
                SettingAction::Sound => {
                    if selected_option.value == 1 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                SettingAction::ChangeName => {
                    if selected_option.value == 2 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                SettingAction::Back => {
                    if selected_option.value == 3 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
        play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_settings_menu_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut selected_option: ResMut<SelectedOption>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_settings: ResMut<GameSettings>,
    mut query: Query<&mut Text, With<OnGameSound>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        debug!("Menu from Settings to Start menu.");
        if selected_option.value == 1 {
            game_settings.is_sound_on = !game_settings.is_sound_on;
            let mut text = query.single_mut();
            text.sections[0].value = "Sound (".to_string() + if game_settings.is_sound_on { "On" } else { "Off" } + ")";
            game_settings.fetch();
        } else if selected_option.value == 2 {
            selected_option.set_value(1);
            menu_state.set(MenuState::ChangeName);
        } else if selected_option.value == 3 {
            selected_option.set_value(1);
            menu_state.set(MenuState::StartMenu);
        }
        play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_start_menu_action(
    keyboard_input: Res<Input<KeyCode>>,
    mut selected_option: ResMut<SelectedOption>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_type: ResMut<GameType>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
    game_settings: ResMut<GameSettings>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        if selected_option.value == 1 {
            selected_option.set_value(1);
            *game_type = GameType::Normal;
            menu_state.set(MenuState::NotInit);
            app_state.set(AppState::Playing);
        } else if selected_option.value == 2 {
            selected_option.set_value(1);
            *game_type = GameType::SpockLizard;
            menu_state.set(MenuState::NotInit);
            app_state.set(AppState::Playing);
        } else if selected_option.value == 3 {
            selected_option.set_value(1);
            *game_type = GameType::FireWater;
            menu_state.set(MenuState::NotInit);
            app_state.set(AppState::Playing);
        } else if selected_option.value == 4 {
            selected_option.set_value(1);
            menu_state.set(MenuState::SettingsMenu);
        } else if selected_option.value == 5 {
            selected_option.set_value(1);
            menu_state.set(MenuState::NotInit);
            app_state.set(AppState::Credits);
        } else if selected_option.value == 6 {
            menu_state.set(MenuState::NotInit);
            app_state.set(AppState::Closing);
        }

        play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
}

#[derive(Event, Debug)]
pub struct OnKeyPressEvent {
    pub key_code: KeyCode,
}

pub fn on_key_press_event_trigger(keyboard_input: Res<Input<KeyCode>>, mut on_key_press_events: EventWriter<OnKeyPressEvent>) {
    for just_pressed in keyboard_input.get_just_pressed() {
        debug!("{:?}", just_pressed);
        on_key_press_events.send(OnKeyPressEvent { key_code: *just_pressed });
    }
}

pub fn on_blinking_text_indicator(mut query: Query<&mut Text, With<OnChangeName>>, time: Res<Time>, mut timer: ResMut<BlinkingTimer>) {
    if timer.tick(time.delta()).finished() {
        let mut text = query.single_mut();
        if text.sections[0].value.contains('|') {
            text.sections[0].value = text.sections[0].value[..text.sections[0].value.len() - 1].to_string();
        } else {
            text.sections[0].value += "|";
        }
    }
}

pub fn on_key_press_event_listener(
    mut on_key_press_events: EventReader<OnKeyPressEvent>,
    mut query: Query<&mut Text, With<OnChangeName>>,
    mut query_border: Query<(&mut BorderColor, &SaveCancelAction), With<SaveCancelAction>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
    mut game_settings: ResMut<GameSettings>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for event in on_key_press_events.iter() {
        debug!("{:?}", event);

        if vec![KeyCode::Left, KeyCode::Right, KeyCode::Space, KeyCode::Return].contains(&event.key_code) {
            for (mut border_color, action) in &mut query_border {
                if vec![KeyCode::Left, KeyCode::Right].contains(&event.key_code) {
                    if border_color.0 == GAME_SELECTED_BORDER_COLOR {
                        *border_color = GAME_NO_SELECTED_BORDER_COLOR.into();
                    } else {
                        *border_color = GAME_SELECTED_BORDER_COLOR.into();
                    }
                } else {
                    if let SaveCancelAction::Save = action {
                        if border_color.0 == GAME_SELECTED_BORDER_COLOR {
                            let text = query.single();
                            game_settings.player_options.name = text.sections[0].value.replace('|', "").clone();
                            game_settings.fetch();
                        }
                    }
                    menu_state.set(MenuState::SettingsMenu)
                }
            }
            play_sound(&audio, game_settings.is_sound_on, &game_sounds.mode_switch);
        } else {
            let mut text = query.single_mut();
            text.sections[0].value = get_new_text_value(text.sections[0].value.clone(), &event.key_code, &audio, &game_sounds, &game_settings);
        }
    }
}

fn get_new_text_value(text: String, pressed: &KeyCode, audio: &Res<Audio>, game_sounds: &Res<GameSounds>, game_settings: &ResMut<GameSettings>) -> String {
    let mut produce_sound_remove_blinking_indicator = true;
    let mut new_text_value = text.clone();
    if pressed == &KeyCode::Back {
        if !text.is_empty() {
            new_text_value = text.replace('|', "");
            new_text_value = new_text_value[..new_text_value.len() - 1].to_string();
        } else {
            produce_sound_remove_blinking_indicator = false;
        }
    } else {
        new_text_value += match *pressed {
            KeyCode::Key1 | KeyCode::Numpad1 => "1",
            KeyCode::Key2 | KeyCode::Numpad2 => "2",
            KeyCode::Key3 | KeyCode::Numpad3 => "3",
            KeyCode::Key4 | KeyCode::Numpad4 => "4",
            KeyCode::Key5 | KeyCode::Numpad5 => "5",
            KeyCode::Key6 | KeyCode::Numpad6 => "6",
            KeyCode::Key7 | KeyCode::Numpad7 => "7",
            KeyCode::Key8 | KeyCode::Numpad8 => "8",
            KeyCode::Key9 | KeyCode::Numpad9 => "9",
            KeyCode::Key0 | KeyCode::Numpad0 => "0",
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
            KeyCode::Mail => "@",
            KeyCode::Apostrophe => "'",
            KeyCode::Colon => ":",
            KeyCode::Minus | KeyCode::NumpadSubtract => "-",
            KeyCode::Plus | KeyCode::NumpadAdd => "+",
            KeyCode::Semicolon => ";",
            KeyCode::Underline => "_",
            KeyCode::Equals | KeyCode::NumpadEquals => "=",
            KeyCode::Comma => ",",
            KeyCode::Period => ".",
            _ => {
                produce_sound_remove_blinking_indicator = false;
                ""
            }
        }
    }

    if produce_sound_remove_blinking_indicator {
        new_text_value = new_text_value.replace('|', "");
        new_text_value = new_text_value[..min(50, new_text_value.len())].to_string();
        play_sound(audio, game_settings.is_sound_on, &game_sounds.mode_switch);
    }
    new_text_value
}
