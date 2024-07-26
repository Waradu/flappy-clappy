use crate::{GameplaySpeed, WindowSize};
use bevy::prelude::*;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OldWindowSize(Vec2::new(0.0, 0.0)));
    }
}

#[derive(Resource)]
struct OldWindowSize(Vec2);
