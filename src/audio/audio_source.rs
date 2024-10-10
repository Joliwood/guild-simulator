use bevy::{audio::PlaybackMode, prelude::*};

pub fn audio_source(my_assets: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(AudioBundle {
            // source: my_assets.load("sounds/Simple-Holidays-V3.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                speed: 1.0,
                ..Default::default()
            },
        })
        .insert(Name::new("Audio source"));
}
