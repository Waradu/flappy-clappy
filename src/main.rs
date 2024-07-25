use bevy::{
    input::common_conditions::input_just_pressed, math::vec2, prelude::*, window::PrimaryWindow,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Playing,
    GameOver,
    Paused,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(f32);

#[derive(Component)]
struct Acceleration(f32);

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct WindowSize(Vec2);

fn main() {
    let window = Window {
        title: "Flappy Clappy".into(),
        name: Some("flappy.app".into()),
        resolution: (800., 500.).into(),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .insert_state(GameState::Playing)
        .insert_resource(WindowSize(Vec2::new(380.0, 500.0)))
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                apply_gravity,
                update_position,
                update_window_size,
                jump.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        Velocity(-1.),
        Acceleration(-9.9),
        Speed(100.),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(100., 100.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
            ..default()
        },
    ));
}

fn apply_gravity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * time.delta_seconds();
    }
}

fn jump(mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();

    velocity.0 = 5.;
}

fn update_position(
    mut query: Query<(&mut Velocity, &Speed, &mut Transform)>,
    time: Res<Time>,
    window_size: Res<WindowSize>,
) {
    for (mut vel, speed, mut transform) in query.iter_mut() {
        transform.translation.y += (vel.0 * time.delta_seconds()) * speed.0;

        let bottom_screen_y = -window_size.0.y / 2.0;

        if transform.translation.y < bottom_screen_y + 50. {
            transform.translation.y = bottom_screen_y + 50.;
            vel.0 = 0.0;
        }
    }
}

fn update_window_size(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>,
) {
    let window = window_query.single_mut();
    window_size.0 = window.size();
}
