use crate::structs::{player_stats::PlayerStats, trigger_structs::ToxicityCountTrigger};
use bevy::prelude::{DetectChanges, Query, Res, Text, With};

pub fn update_toxicity_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<ToxicityCountTrigger>>,
) {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            // text.sections[0].value = player_stats.toxicity.to_string();
        }
    }
}
