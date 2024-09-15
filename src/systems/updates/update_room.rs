use crate::{
    enums::RoomEnum,
    structs::{
        general_structs::{Missions, PlayerStats, SelectedRecruit},
        trigger_structs::ResetRoomTrigger,
    },
    ui::{
        interface::gold_counter::MyAssets,
        rooms::{
            room_barrack::room_barrack, room_command_room::room_command_room,
            room_office::room_office, room_store::room_store,
        },
    },
};
use bevy::prelude::*;

/// Checks for changes in PlayerStats and updates the room accordingly
///
/// # Parameters
/// - `asset_server`: Bevy's asset server to load assets
/// - `player_stats`: The player stats to determine the current room
/// - `commands`: Bevy's commands to spawn/despawn entities
/// - `query`: Query to find and despawn existing room entities
pub fn update_room(
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    query: Query<Entity, With<ResetRoomTrigger>>,
    selected_recruit: Res<SelectedRecruit>,
    missions: Res<Missions>,
    image_assets: Res<MyAssets>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if player_stats.is_changed() || selected_recruit.is_changed() {
        // Despawn existing room entities marked with ResetRoomTrigger only if player_stats.room has changed
        for entity in query.iter() {
            info!("PlayerStats has changed");
            commands.entity(entity).despawn_recursive();
        }

        // Spawn new room based on player_stats
        match player_stats.room {
            RoomEnum::Office => room_office(&asset_server, &mut commands, texture_atlas_layouts),
            RoomEnum::Barrack => room_barrack(
                &asset_server,
                &mut commands,
                &player_stats,
                &selected_recruit,
                &image_assets,
            ),
            RoomEnum::Store => room_store(&asset_server, &mut commands),
            RoomEnum::CommandRoom => {
                room_command_room(&asset_server, &mut commands, missions, &image_assets)
            }
        }
    }
}
