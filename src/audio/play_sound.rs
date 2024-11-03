use crate::{enums::SoundEnum, my_assets::MyAssets};
use bevy::{
    audio::{AudioPlayer, AudioSource, PlaybackMode, PlaybackSettings, Volume},
    prelude::{Commands, Res},
};

pub fn play_sound(my_assets: &Res<MyAssets>, commands: &mut Commands, sound_enum: SoundEnum) {
    let audio = my_assets.load_sound(sound_enum);
    commands
        .spawn(AudioPlayer::<AudioSource>(audio))
        .insert(PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.1),
            speed: 1.0,
            ..Default::default()
        });
}
