use crate::structs::{player_stats::PlayerStats, trigger_structs::ReputationCountTrigger};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_reputation_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<ReputationCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if player_stats.is_changed() {
        *writer.text(*query, 0) = player_stats.reputation.to_string();
    }
}
