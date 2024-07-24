use bevy::{prelude::*, window::WindowLevel};

#[derive(Resource)]
struct EventTimer(Timer);

fn main() {
    let window = Window {
        title: "Otaku".into(),
        name: Some("otaku.app".into()),
        transparent: true,
        decorations: false,
        skip_taskbar: true,
        window_level: WindowLevel::AlwaysOnTop,
        resolution: (300., 300.).into(),
        ..default()
    };

    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }),))
        .insert_resource(EventTimer(Timer::from_seconds(10., TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("./pet/idle/default/normal/1/000.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(300., 300.)),
            ..default()
        },
        texture,
        ..default()
    });
}
