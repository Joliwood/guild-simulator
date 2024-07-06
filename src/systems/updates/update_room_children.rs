use crate::enums::RoomEnum;
use crate::structs::{PlayerStats, ResetRoomTrigger};
use crate::ui::rooms::{
    room_barrack::room_barrack, room_office::room_office, room_store::room_store,
};
use bevy::prelude::*;

pub fn update_room_children(
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    query: Query<Entity, With<ResetRoomTrigger>>,
) {
    // if player_stats.room.is_added() {
    //     // Despawn existing room entities marked with ResetRoomTrigger
    //     room_office(&asset_server, &mut commands);
    // }

    if player_stats.room.is_changed() {
        for parent in query.iter() {
            // Despawn existing children
            commands.entity(parent).despawn_descendants();

            // Spawn new room based on player_stats
            match player_stats.room {
                RoomEnum::Office => room_office(&asset_server, &mut commands),
                RoomEnum::Barrack => room_barrack(&asset_server, &mut commands),
                RoomEnum::Store => room_store(&asset_server, &mut commands),
                RoomEnum::CommandRoom => todo!(),
            }
        }
    }
}
