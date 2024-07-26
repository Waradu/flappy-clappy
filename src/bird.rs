use bevy::prelude::*;
use crate::{GameState, WindowSize, has_user_input};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    apply_gravity.run_if(in_state(GameState::Playing)),
                    update_position.run_if(in_state(GameState::Playing)),
                    jump.run_if(has_user_input),
                    change_texture,
                ),
            );
    }
}

#[derive(Resource)]
pub struct BirdTextures {
    pub downflap: Handle<Image>,
    pub midflap: Handle<Image>,
    pub upflap: Handle<Image>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Resource, Component, Deref, DerefMut)]
pub struct TextureTimer(pub Timer);

#[derive(Component)]
pub struct CurrentTexture(pub u8);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let downflap: Handle<Image> = asset_server.load("bird/downflap.png");
    let midflap: Handle<Image> = asset_server.load("bird/midflap.png");
    let upflap: Handle<Image> = asset_server.load("bird/upflap.png");

    commands.insert_resource(BirdTextures {
        downflap: downflap.clone(),
        midflap: midflap.clone(),
        upflap: upflap.clone(),
    });

    commands.spawn((
        Player,
        Velocity(-1.),
        Acceleration(-15.),
        Speed(100.),
        CurrentTexture(0),
        Jump(7.),
        TextureTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(68., 48.)),
                ..default()
            },
            texture: downflap,
            transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
            ..default()
        },
    ));
}

pub fn change_texture(
    time: Res<Time>,
    bird_textures: Res<BirdTextures>,
    mut query: Query<(&mut CurrentTexture, &mut Handle<Image>, &mut TextureTimer)>,
) {
    for (mut current_texture, mut handle, mut timer) in query.iter_mut() {
        let textures = [
            &bird_textures.downflap,
            &bird_textures.midflap,
            &bird_textures.upflap,
            &bird_textures.midflap,
        ];

        timer.tick(time.delta());
        if timer.finished() {
            current_texture.0 = (current_texture.0 + 1) % textures.len() as u8;
            *handle = textures[current_texture.0 as usize].clone();
        }
    }
}

pub fn apply_gravity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * time.delta_seconds();
    }
}

pub fn jump(mut query: Query<(&mut Velocity, &Jump), With<Player>>) {
    let (mut velocity, strength) = query.single_mut();

    velocity.0 = strength.0;
}

pub fn update_position(
    mut query: Query<(&mut Velocity, &Speed, &mut Transform, &Sprite)>,
    time: Res<Time>,
    window_size: Res<WindowSize>,
) {
    for (mut vel, speed, mut transform, sprite) in query.iter_mut() {
        transform.translation.y += (vel.0 * time.delta_seconds()) * speed.0;

        match vel.0 {
            _ if vel.0 < -2. => {
                transform.rotation = Quat::from_rotation_z(-45.0);
            }
            _ if vel.0 > 2. => {
                transform.rotation = Quat::from_rotation_z(45.0);
            }
            _ => {
                transform.rotation = Quat::from_rotation_z(0.0);
            }
        }

        let bottom_screen_y = -window_size.0.y / 2.0;
        let top_screen_y = window_size.0.y / 2.0;
        let size = sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));

        if transform.translation.y < bottom_screen_y + size.y / 2. {
            transform.translation.y = bottom_screen_y + size.y / 2.;
            vel.0 = 0.0;
        }

        if transform.translation.y > top_screen_y - size.y / 2. {
            transform.translation.y = top_screen_y - size.y / 2.;
            vel.0 = 0.0;
        }
    }
}
