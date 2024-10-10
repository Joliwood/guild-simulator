use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Commands, Res, ResMut},
    sprite::TextureAtlasLayout,
};

use crate::ui::{interface::gold_counter::MyAssets, rooms::room_office::room_office};

/// # All the UI logic and components will be setup here
///
/// It will load the office room at start
///
/// ## Parameters
/// - `commands`: Bevy's commands
/// - `asset_server`: Bevy's asset server
pub fn room_setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    my_assets: Res<MyAssets>,
) {
    room_office(
        my_assets,
        &asset_server,
        &mut commands,
        &mut texture_atlas_layouts,
    );
}
