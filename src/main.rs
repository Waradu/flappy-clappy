use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod background;
mod pipe;
mod bird;

use background::BackgroundPlugin;
use pipe::PipePlugin;
use bird::BirdPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Playing,
    GameOver,
    Paused,
}

#[derive(Resource)]
struct WindowSize(Vec2);

#[derive(Resource)]
struct GameplaySpeed(f32);

fn main() {
    let window = Window {
        title: "Flappy Clappy".into(),
        name: Some("flappy.app".into()),
        resolution: (800., 500.).into(),
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                }),
            BirdPlugin,
            BackgroundPlugin,
        ))
        .insert_state(GameState::Playing)
        .insert_resource(WindowSize(Vec2::new(380.0, 500.0)))
        .insert_resource(GameplaySpeed(200.0))
        .add_systems(Startup, update_window_size)
        .add_systems(Update, update_window_size)
        .run();
}

fn update_window_size(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>,
) {
    let window = window_query.single_mut();
    window_size.0 = window.size();
}

pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}
