use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
    window::{PrimaryWindow, WindowLevel},
};

#[derive(Resource)]
struct Dragging(pub bool);

fn main() {
    let window = Window {
        title: "Otaku".into(),
        name: Some("otaku.app".into()),
        transparent: true,
        decorations: false,
        skip_taskbar: true,
        window_level: WindowLevel::AlwaysOnTop,
        resolution: (300., 300.).into(),
        position: WindowPosition::At(IVec2::new(10, 10)),
        ..default()
    };

    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }),))
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Dragging(false))
        .add_systems(
            Update,
            (
                window_position.run_if(input_just_pressed(KeyCode::Space)),
                start_drag.run_if(input_just_pressed(MouseButton::Left)),
                end_drag.run_if(input_just_released(MouseButton::Left)),
                drag.run_if(dragging_true),
            ).chain(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("./pet/notfound.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(250., 250.)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn window_position(mut q_primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_primary_window.single_mut();

    

    if let WindowPosition::At(mut pos) = window.position {
        pos.x += 20;
        pos.y += 20;
        window.position = WindowPosition::At(pos);
    }
}

fn dragging_true(dragging: Res<Dragging>) -> bool {
    dragging.0
}

fn start_drag(mut dragging: ResMut<Dragging>) {
    dragging.0 = true;
}

fn end_drag(mut dragging: ResMut<Dragging>) {
    dragging.0 = false;
}

fn drag() {
    
}
