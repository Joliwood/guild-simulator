use bevy::{audio::PlaybackMode, prelude::*};

use crate::{enums::SoundEnum, my_assets::MyAssets};

pub fn audio_source(my_assets: Res<MyAssets>, mut commands: Commands) {
    let audio = my_assets.load_sound(SoundEnum::SimpleHolidaysV3);
    commands
        .spawn(AudioBundle {
            source: audio,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                speed: 1.0,
                ..Default::default()
            },
        })
        .insert(Name::new("Audio source"));
}
