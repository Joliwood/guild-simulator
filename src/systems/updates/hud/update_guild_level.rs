use crate::structs::{player_stats::PlayerStats, trigger_structs::GuildLvlTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_guild_level(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<GuildLvlTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = format!("{} : {}", t!("day"), player_stats.guild_level);
}
