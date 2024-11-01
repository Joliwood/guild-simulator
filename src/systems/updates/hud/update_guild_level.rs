use crate::structs::{player_stats::PlayerStats, trigger_structs::GuildLvlTrigger};
use bevy::{
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_guild_level(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<GuildLvlTrigger>>,
) {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[1].value = player_stats.guild_level.to_string();
        }
    }
}
