use crate::{enums::SoundEnum, my_assets::MyAssets};
use bevy::{
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    prelude::{default, Commands, Res},
};

pub fn play_sound(my_assets: &Res<MyAssets>, commands: &mut Commands, sound_enum: SoundEnum) {
    let audio = my_assets.load_sound(sound_enum);
    commands.spawn(AudioBundle {
        source: audio,
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.1),
            speed: 1.0,
            ..default()
        },
    });
}
