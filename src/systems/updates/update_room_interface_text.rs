use crate::structs::{PlayerStats, PlayerStatsRoomTrigger};
use bevy::{
    log::error,
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the room interface text (top center of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_room_interface_text(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<PlayerStatsRoomTrigger>>,
) -> () {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            error!("we are changing text in update room interface text only normally");
            text.sections[0].value = format!("{:?}", player_stats.room);
        }
    }
}
