use crate::{
    enums::RoomEnum,
    structs::{
        PlayerStats, PlayerStatsRecruitsTrigger, RecruitStats, RecruitsTrigger, ResetRoomTrigger,
        UniqueId,
    },
    ui::rooms::{
        room_barrack::room_barrack, room_command_room::room_command_room, room_office::room_office,
        room_store::room_store,
    },
};
use bevy::prelude::*;
use uuid::Uuid;

/// Checks for changes in PlayerStats and updates the room accordingly
///
/// # Parameters
/// - `asset_server`: Will be necessary go get texture frame if it up in level for exemple, or health status if in recovery
/// - `player_stats`: The trigger when the player_stats.recruits is triggered with is_changed()
/// - `commands`: Bevy's commands to spawn/despawn entities
/// - `query`: Struct trigger to see which node has to be updated
pub fn update_recruits(
    // asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    mut query: Query<Entity, With<RecruitsTrigger>>,
) -> () {
    // 1 - We find which recruit had been updated (with is_changed())
    // 2 - We update the recruit with the new stats
    // OR
    // 2 - We despawn each node and respawn it with the new stats

    // if player_sCtats.recruits.is_changed() {
    for entity in query.iter() {
        println!("\n PlayerStats has changed");
        //     commands.entity(entity).despawn_recursive();
    }
    // }

    // for entity in query.iter() {
    //     println!("\n PlayerStats has changed");
    //     // commands.entity(entity).despawn_recursive();
    // }
}
