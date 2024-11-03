use crate::{enums::SoundEnum, my_assets::MyAssets};
use bevy::{audio::PlaybackMode, prelude::*};

pub fn audio_source(my_assets: Res<MyAssets>, mut commands: Commands) {
    let audio = my_assets.load_sound(SoundEnum::SimpleHolidaysV3);
    commands
        .spawn(AudioPlayer::<AudioSource>(audio))
        .insert(PlaybackSettings {
            mode: PlaybackMode::Loop,
            paused: true,
            speed: 1.0,
            ..default()
        })
        .insert(Name::new("Audio source"));
}
