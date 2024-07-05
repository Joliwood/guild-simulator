use crate::structs::{GoldCountText, PlayerStats};
use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};

pub fn update_gold_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<GoldCountText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", player_stats.golds);
    }
}
