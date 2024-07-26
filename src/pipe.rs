use crate::Despawn;

use super::{bird::Player, GameState, GameplaySpeed, WindowSize};
use bevy::{
    asset::embedded_asset, math::bounding::{Aabb2d, IntersectsVolume}, prelude::*
};
use rand::Rng;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (scroll, spawn_pipe, check_pipe_collision))
            .insert_resource(PipeTimer(Timer::from_seconds(1.5, TimerMode::Repeating)));

        embedded_asset!(app, "bird/pipe/green.png")
    }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Resource, Component, Deref, DerefMut)]
pub struct PipeTimer(pub Timer);

pub fn spawn_pipe(
    mut commands: Commands,
    asser_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<PipeTimer>,
    window_size: Res<WindowSize>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let texture = asser_server.load("embedded://flappy_clappy/bird/pipe/green.png");

    let mut rng = rand::thread_rng();
    let y = rng.gen_range(-150.0..150.0);

    commands.spawn((
        Pipe,
        Despawn,
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_xyz(window_size.0.x / 2., y - 320.0, 1.)
                .with_scale(Vec3::new(1.5, 1.5, 1.)),
            ..Default::default()
        },
    ));

    commands.spawn((
        Pipe,
        Despawn,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(window_size.0.x / 2., y + 320.0, 1.)
                .with_scale(Vec3::new(1.5, 1.5, 1.)),
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn scroll(
    mut query: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>,
    speed: Res<GameplaySpeed>,
) {
    for mut transform in &mut query {
        transform.translation.x -= speed.0 * time.delta_seconds();
    }
}

pub fn check_pipe_collision(
    mut state: ResMut<NextState<GameState>>,
    bird: Query<&Transform, With<Player>>,
    pipes: Query<&Transform, With<Pipe>>,
) {
    if bird.iter().len() != 1 {
        return;
    }

    let bird = bird.single();
    for pipe in &pipes {
        let collision = Aabb2d::new(bird.translation.xy(), Vec2::new(68., 48.) / 2.00).intersects(
            &Aabb2d::new(pipe.translation.xy(), (Vec2::new(52., 320.) * 1.5) / 2.0),
        );
        if collision {
            state.set(GameState::GameOver);
        }
    }
}
