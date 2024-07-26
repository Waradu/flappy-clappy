use crate::{GameplaySpeed, WindowSize};
use bevy::{asset::embedded_asset, prelude::*};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Update, (move_background, resize_background))
            .insert_resource(OldWindowSize(Vec2::new(0.0, 0.0)));

        embedded_asset!(app, "bird/background/day.png")
    }
}

#[derive(Resource)]
pub struct OldWindowSize(Vec2);

#[derive(Component)]
pub struct Background;

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    let background = asset_server.load("embedded://flappy_clappy/bird/background/day.png");

    let texture_size = Vec2::new(288.0, 512.0);
    let cols = (window_size.0.x / texture_size.x).ceil() as i32 + 2;

    for col in 0..cols {
        let scale = window_size.0.y / texture_size.y;

        let x_position = col as f32 * (texture_size.x * scale)
            - (window_size.0.x / 2.0)
            - ((texture_size.x * scale) / 2.0);

        commands.spawn((
            Background,
            SpriteBundle {
                texture: background.clone(),
                transform: Transform {
                    translation: Vec3::new(x_position, 0.0, -0.1),
                    scale: Vec3::new(scale, scale, 1.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn move_background(
    time: Res<Time>,
    speed: Res<GameplaySpeed>,
    mut query: Query<&mut Transform, With<Background>>,
    window_size: Res<WindowSize>,
) {
    let texture_size = Vec2::new(288.0, 512.0);

    for mut transform in query.iter_mut() {
        transform.translation.x -= (speed.0 - 50.) * time.delta_seconds();

        let texture_size_x = texture_size.x * transform.scale.x;

        if transform.translation.x <= -((window_size.0.x + texture_size_x) / 2.) {
            transform.translation.x +=
                texture_size_x * ((window_size.0.x / texture_size_x).ceil() + 1.);
        }
    }
}

pub fn resize_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
    mut old_window_size: ResMut<OldWindowSize>,
    query: Query<Entity, With<Background>>,
) {
    if old_window_size.0 == window_size.0 {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    old_window_size.0 = window_size.0;
    spawn_background(commands, asset_server, window_size);
}
