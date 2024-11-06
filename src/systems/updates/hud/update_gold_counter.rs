use crate::structs::{player_stats::PlayerStats, trigger_structs::GoldCountTrigger};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_gold_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<GoldCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if player_stats.is_changed() {
        *writer.text(*query, 0) = player_stats.golds.to_string();
    }
}
