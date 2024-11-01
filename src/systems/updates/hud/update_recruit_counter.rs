use crate::structs::{player_stats::PlayerStats, trigger_structs::RecruitCountTrigger};
use bevy::{
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

pub fn update_recruit_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<RecruitCountTrigger>>,
) {
    if player_stats.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = player_stats.recruits.len().to_string();
        }
    }
}
