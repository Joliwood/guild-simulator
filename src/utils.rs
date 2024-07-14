use bevy::prelude::ResMut;

use crate::{
    enums::{RoomDirectionEnum, RoomEnum},
    structs::PlayerStats,
};

/// Determines the new room based on the given direction and current player stats.
///
/// # Parameters
/// - `player_stats`: The current player stats containing the current room.
/// - `direction`: The direction in which the room change is requested.
///
/// # Returns
/// The new room enum corresponding to the direction.
pub fn get_new_room(
    player_stats: &ResMut<PlayerStats>,
    direction: RoomDirectionEnum,
) -> Option<RoomEnum> {
    match player_stats.room {
        RoomEnum::Office => match direction {
            RoomDirectionEnum::Right => Some(RoomEnum::Barrack),
            RoomDirectionEnum::Left => Some(RoomEnum::Store),
            RoomDirectionEnum::Bottom => Some(RoomEnum::CommandRoom),
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::Barrack => match direction {
            RoomDirectionEnum::Right => None,
            RoomDirectionEnum::Left => Some(RoomEnum::Office),
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::Store => match direction {
            RoomDirectionEnum::Right => Some(RoomEnum::Office),
            RoomDirectionEnum::Left => None,
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::CommandRoom => match direction {
            RoomDirectionEnum::Right => None,
            RoomDirectionEnum::Left => None,
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => Some(RoomEnum::Office),
        },
    }
}

pub fn increment_golds(player_stats: &mut ResMut<PlayerStats>, amount: i32) {
    println!(
        "Incrementing golds by {} for a total of : {}",
        amount, player_stats.golds,
    );
    player_stats.golds += amount;
}
