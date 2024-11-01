use crate::structs::{player_stats::PlayerStats, trigger_structs::ReputationCountTrigger};
use bevy::{
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_reputation_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<ReputationCountTrigger>>,
) {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = player_stats.reputation.to_string();
        }
    }
}
