use bevy::prelude::{Camera2d, Commands};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
