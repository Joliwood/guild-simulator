use crate::structs::{general_structs::PlayerStats, trigger_structs::SleepButtonTrigger};
use bevy::prelude::*;

pub fn sleep_button_system(
    mut interaction_query: Query<(&Interaction, &SleepButtonTrigger), Changed<Interaction>>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            // Increment the day in player_stats
            player_stats.day += 1;
            info!("Day updated to: {}", player_stats.day);
        }
    }
}
