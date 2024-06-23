use bevy::{audio::PlaybackMode, prelude::*};

pub fn audio_source(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Simple-Holidays-V3.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            paused: true,
            speed: 1.0,
            ..Default::default()
        },
        ..default()
    });
}
