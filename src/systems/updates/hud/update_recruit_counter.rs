use crate::structs::{player_stats::PlayerStats, trigger_structs::RecruitCountTrigger};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_recruit_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<RecruitCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if player_stats.is_changed() {
        *writer.text(*query, 0) = player_stats.recruits.len().to_string();
    }
}
