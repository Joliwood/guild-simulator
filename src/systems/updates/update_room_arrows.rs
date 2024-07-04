use crate::structs::{PlayerStats, UniqueId};
use bevy::{
    prelude::{Component, Query, Res, With},
    text::Text,
};

pub fn update_room_arrows(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<UniqueId>>,
) {
    for mut text in query.iter_mut() {
        print!("{:#?}", player_stats.room);
        // text.sections[0].value = format!("{:?}", player_stats.room);
    }
}
