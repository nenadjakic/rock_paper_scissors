use bevy::app::AppExit;
use bevy::prelude::*;

use crate::common::*;

#[derive(Component)]
pub struct OnClosingScreen;

#[derive(Component)]
pub struct ClosingText;

#[derive(Resource, Deref, DerefMut)]
pub struct ClosingTimer(Timer);

pub struct ClosingPlugin;

impl Plugin for ClosingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Closing), setup_closing_screen);
        app.add_systems(Update, closing.run_if(in_state(AppState::Closing)));
    }
}

fn setup_closing_screen(mut commands: Commands, game_font: Res<GameFont>) {
    let font = &game_font.0;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnClosingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Bye, bye",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: TITLE_COLOR,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
                ClosingText,
            ));
        });

    commands.insert_resource(ClosingTimer(Timer::from_seconds(CLOSING_DURATION, TimerMode::Once)));
}

fn closing(mut query: Query<&mut Text, With<ClosingText>>, mut app_exit_events: EventWriter<AppExit>, time: Res<Time>, mut timer: ResMut<ClosingTimer>) {
    let min = 0.0;
    let max = CLOSING_DURATION;

    if timer.tick(time.delta()).finished() {
        app_exit_events.send(AppExit);
    } else {
        for mut text in &mut query {
            time.delta();
            let milliseconds = timer.elapsed_secs();

            let alpha = 1.0 - ((milliseconds - min) / (max - min));

            let mut new_color = TITLE_COLOR;
            new_color.set_a(alpha);
            text.sections[0].style.color = new_color;
        }
    }
}
