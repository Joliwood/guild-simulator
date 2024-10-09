use crate::enums::SoundEnum;
use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    prelude::{default, Commands, Res},
};

pub fn play_sound(asset_server: &Res<AssetServer>, commands: &mut Commands, asset: SoundEnum) {
    let path: &str = asset.get_path();
    commands.spawn(AudioBundle {
        source: asset_server.load(path),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.1),
            speed: 1.0,
            ..default()
        },
    });
}
