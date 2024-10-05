use super::office::room_office::room_office;
use crate::ui::interface::gold_counter::MyAssets;
use bevy::prelude::{Commands, Res};

/// # All the UI logic and components will be setup here
///
/// It will load the office room at start
///
/// ## Parameters
/// - `commands`: Bevy's commands
/// - `my_assets`: Bevy's asset server
pub fn room_setup(my_assets: Res<MyAssets>, mut commands: Commands) {
    room_office(&my_assets, &mut commands);
}
