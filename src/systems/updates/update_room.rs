use crate::{
    enums::RoomEnum,
    structs::{PlayerStats, ResetRoomTrigger},
    ui::rooms::{room_barrack::room_barrack, room_office::room_office, room_store::room_store},
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
    mut query: Query<Entity, With<ResetRoomTrigger>>,
) {
    if player_stats.is_changed() {
        // Despawn existing room entities marked with ResetRoomTrigger
        for entity in query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        // Spawn new room based on player_stats
        match player_stats.room {
            RoomEnum::Office => room_office(&asset_server, &mut commands),
            RoomEnum::Barrack => room_barrack(&asset_server, &mut commands),
            RoomEnum::Store => room_store(&asset_server, &mut commands),
            RoomEnum::CommandRoom => todo!("Implement CommandRoom"),
        }
    }
}
