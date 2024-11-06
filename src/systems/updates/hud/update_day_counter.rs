use crate::structs::{player_stats::PlayerStats, trigger_structs::PlayerDayTrigger};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_day_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<PlayerDayTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if player_stats.is_changed() {
        *writer.text(*query, 0) = format!("Day : {}", player_stats.day);
    }
}
