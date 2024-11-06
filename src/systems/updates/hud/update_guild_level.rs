use crate::structs::{player_stats::PlayerStats, trigger_structs::GuildLvlTrigger};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_guild_level(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<GuildLvlTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if player_stats.is_changed() {
        *writer.text(*query, 0) = player_stats.guild_level.to_string();
    }
}
