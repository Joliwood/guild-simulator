use crate::structs::{
    general_structs::{MissionModalVisible, UniqueId},
    missions::{Mission, Missions, SelectedMission},
};
use bevy::prelude::*;

/// Select the mission when the button is pressed
///
/// - 1 - We get the ID from the unique id inserted in the node button
/// - 2 - We assign with this ID the selected mission
/// - 3 - We open de details mission modal
pub fn select_mission_button(
    mut interaction_query: Query<
        (&Interaction, &UniqueId, &Mission, &mut Transform),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    _missions: Res<Missions>,
    mut selected_mission: ResMut<SelectedMission>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
) {
    let _window = windows.single_mut();
    if !mission_modal_visibility.0 {
        for (interaction, unique_id, mission, mut transform) in &mut interaction_query {
            // WIP - Change the method to query it
            if unique_id.0 == "select_mission_button" {
                match *interaction {
                    Interaction::Pressed => {
                        transform.scale = Vec3::splat(1.0);
                        selected_mission.mission_id = Some(mission.id);
                        mission_modal_visibility.0 = true;
                    }
                    Interaction::Hovered => {
                        transform.scale = Vec3::splat(1.05);
                    }
                    Interaction::None => {
                        transform.scale = Vec3::splat(1.0);
                    }
                }
            }
        }
    }
}
