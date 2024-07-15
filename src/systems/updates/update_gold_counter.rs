use crate::structs::{GoldCountTrigger, PlayerStats};
use bevy::{
    log::info,
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_gold_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<GoldCountTrigger>>,
) -> () {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            info!("MOAR GOLDS");
            text.sections[0].value = format!("{}", player_stats.golds);
        }
    }
}
