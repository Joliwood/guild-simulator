use crate::structs::{PlayerStats, PlayerStatsRoomText};
use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};

pub fn update_room_interface_text(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<PlayerStatsRoomText>>,
) {
    // ! The query tick every second
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:?}", player_stats.room);
    }
}
