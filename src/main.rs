use bevy::{prelude::*, window::WindowLevel};

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
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
