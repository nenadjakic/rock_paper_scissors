use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::common::*;
use crate::game_type::GameType;

#[derive(Component)]
pub struct OnStartMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

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
    Back,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), init_setup_menu)
            .add_systems(OnEnter(MenuState::StartMenu), setup_start_menu)
            .add_systems(Update, (switch_start_menu_action, confirm_start_menu_action).run_if(in_state(MenuState::StartMenu)))
            .add_systems(OnExit(MenuState::StartMenu), despawn_screen::<OnStartMenuScreen>)
            .add_systems(OnEnter(MenuState::SettingsMenu), setup_setting_menu)
            .add_systems(Update, (switch_settings_menu_action, confirm_settings_menu_action).run_if(in_state(MenuState::SettingsMenu)))
            .add_systems(OnExit(MenuState::SettingsMenu), despawn_screen::<OnSettingsMenuScreen>);
    }
}

pub fn init_setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    debug!("menu::init_setup");
    menu_state.set(MenuState::StartMenu);
}

pub fn setup_start_menu(mut commands: Commands, game_font: Res<GameFont>, game_images: Res<GameImages>) {
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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnStartMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                }).with_children(|parent| {
                // Display the game name
                parent.spawn(
                    TextBundle::from_section(
                        TITLE_TEXT,
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
                spawn_start_menu_button(parent, Visibility::Visible, "Normal", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::Normal, &font);

                spawn_start_menu_button(parent, Visibility::Hidden, "Spock lizard", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::SpockLizard, &font);

                spawn_start_menu_button(parent, Visibility::Hidden, "Fire water", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::FireWater, &font);

                spawn_start_menu_button(parent, Visibility::Hidden, "Settings", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::Settings, &font);

                spawn_start_menu_button(parent, Visibility::Hidden, "Credits", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::Credits, &font);

                spawn_start_menu_button(parent, Visibility::Hidden, "Exit", &button_style,
                                        &button_icon_style, &game_images.joystick, MenuAction::Exit, &font);
            });
        });
}

pub fn setup_setting_menu(mut commands: Commands, game_font: Res<GameFont>, game_images: Res<GameImages>,
                          settings: Res<GameSettings>) {
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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        )).with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                background_color: MENU_BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Settings",
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

            parent.spawn(
                NodeBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage::new(game_images.joystick.clone()),
                        visibility: Visibility::Visible,
                        ..default()
                    }, SettingAction::Sound));

                    parent.spawn((TextBundle::from_section(
                        "Sound (".to_string() + if settings.is_sound_on { "On" } else { "Off" } + ")",
                        TextStyle {
                            font_size: BUTTON_TEXT_SIZE,
                            color: BUTTON_TITLE_COLOR,
                            font: font.clone(),
                        },
                    ).with_style(Style {
                        margin: UiRect::left(Val::Px(10.0)),
                        ..default()
                    }), OnGameSound));
                });

            parent.spawn(
                NodeBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage::new(game_images.joystick.clone()),
                        visibility: Visibility::Hidden,
                        ..default()
                    }, SettingAction::Back));
                    parent.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font_size: BUTTON_TEXT_SIZE,
                            color: BUTTON_TITLE_COLOR,
                            font: font.clone(),
                        },
                    ).with_style(Style {
                        margin: UiRect::left(Val::Px(10.0)),
                        ..default()
                    }));
                });

        });
    });
}

fn spawn_start_menu_button(parent: &mut ChildBuilder, visibility: Visibility, text: &str,
                           button_style: &Style,
                           icon_style: &Style, icon: &Handle<Image>,
                           menu_action: MenuAction,
                           font: &Handle<Font>
) {
    parent.spawn(
        NodeBundle {
            style: button_style.clone(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                style: icon_style.clone(),
                image: UiImage::new(icon.clone()),
                visibility,
                ..default()
            }, menu_action));
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: BUTTON_TEXT_SIZE,
                    color: BUTTON_TITLE_COLOR,
                    font: font.clone(),
                },
            ).with_style(Style {
                margin: UiRect::left(Val::Px(10.0)),
                ..default()
            }));
        });
}

pub fn switch_start_menu_action(keyboard_input: Res<Input<KeyCode>>, mut game_type: ResMut<GameType>,
                                mut query: Query<(&mut Visibility, &MenuAction), With<MenuAction>>,
                                audio: Res<Audio>,
                                game_sounds: Res<GameSounds>,
                                mut selected_option: ResMut<SelectedOption>,
                                settings: Res<GameSettings>,
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

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn switch_settings_menu_action(keyboard_input: Res<Input<KeyCode>>,
                                   mut query: Query<(&mut Visibility, &SettingAction), With<SettingAction>>,
                                   audio: Res<Audio>,
                                   game_sounds: Res<GameSounds>,
                                   mut selected_option: ResMut<SelectedOption>,
                                   settings: Res<GameSettings>,
) {
    let mut up_or_down = false;
    if keyboard_input.just_pressed(KeyCode::Up) {
        up_or_down = true;

        if selected_option.value > 1 {
            selected_option.value -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        up_or_down = true;

        if selected_option.value < 2 {
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
                SettingAction::Back => {
                    if selected_option.value == 2 {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_settings_menu_action(keyboard_input: Res<Input<KeyCode>>,
                                    mut selected_option: ResMut<SelectedOption>,
                                    mut menu_state: ResMut<NextState<MenuState>>,
                                    mut settings: ResMut<GameSettings>,
                                    mut query: Query<&mut Text, With<OnGameSound>>,
                                    audio: Res<Audio>,
                                    game_sounds: Res<GameSounds>
) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        debug!("Menu from Settings to Start menu.");
        if selected_option.value == 1 {
            settings.is_sound_on = !settings.is_sound_on;
            let mut text = query.single_mut();
            text.sections[0].value = "Sound (".to_string() + if settings.is_sound_on { "On" } else { "Off" } + ")";
        } else if selected_option.value == 2 {
            selected_option.set_value(1);
            menu_state.set(MenuState::StartMenu)
        }
        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}

pub fn confirm_start_menu_action(keyboard_input: Res<Input<KeyCode>>, mut selected_option: ResMut<SelectedOption>,
                                 mut app_state: ResMut<NextState<AppState>>,
                                 mut menu_state: ResMut<NextState<MenuState>>,
                                 mut game_type: ResMut<GameType>,
                                 audio: Res<Audio>,
                                 game_sounds: Res<GameSounds>,
                                 settings: ResMut<GameSettings>,
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

        play_sound(&audio, settings.is_sound_on, &game_sounds.mode_switch);
    }
}