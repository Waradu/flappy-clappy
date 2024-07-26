use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod background;
mod bird;
mod pipe;

use background::BackgroundPlugin;
use bird::BirdPlugin;
use pipe::PipePlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Playing,
    GameOver,
    _Paused,
}

#[derive(Component)]
pub struct Despawn;

#[derive(Resource)]
pub struct WindowSize(Vec2);

#[derive(Resource)]
pub struct GameplaySpeed(f32);

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
            PipePlugin,
            BirdPlugin,
            BackgroundPlugin,
        ))
        .insert_state(GameState::Playing)
        .insert_resource(WindowSize(Vec2::new(380.0, 500.0)))
        .insert_resource(GameplaySpeed(200.0))
        .add_systems(Startup, update_window_size)
        .add_systems(Update, update_window_size)
        .add_systems(OnEnter(GameState::GameOver), reset)
        .run();
}

fn update_window_size(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>,
) {
    if window_query.iter().len() != 1 {
        return;
    }

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

pub fn reset(mut commands: Commands, mut state: ResMut<NextState<GameState>>, query: Query<Entity, With<Despawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    state.set(GameState::Playing);
}
