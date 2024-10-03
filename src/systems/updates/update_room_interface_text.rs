use crate::structs::{general_structs::PlayerStats, trigger_structs::PlayerStatsRoomTrigger};
use bevy::{
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
            text.sections[0].value = format!(
                "Nb of recruits : {} | Day {:?}",
                player_stats.recruits.len(),
                player_stats.day
            );
        }
    }
}
