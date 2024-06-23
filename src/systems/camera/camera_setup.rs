use bevy::prelude::{Camera2dBundle, Commands};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
