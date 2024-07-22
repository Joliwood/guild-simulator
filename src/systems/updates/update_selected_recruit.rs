use crate::structs::{SelectedMission, SelectedMissionTrigger};
use bevy::{
    log::info,
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_recruit_infos(
    selected_mission: Res<SelectedMission>,
    mut query: Query<&mut Text, With<SelectedMissionTrigger>>,
) -> () {
    if selected_mission.is_changed() {
        info!(
            "the recruit assigned to the selected mission is now : {:?}",
            selected_mission
        );
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{:?}", selected_mission.recruit_id);
        }
    }
}
