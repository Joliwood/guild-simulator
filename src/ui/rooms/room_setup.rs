use bevy::{
    asset::Assets,
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
/// - `my_assets`: Bevy's asset server
pub fn room_setup(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    room_office(&my_assets, &mut commands, &mut texture_atlas_layouts);
}
