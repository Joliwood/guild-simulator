use crate::{
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::{Mission, Missions, SelectedMission},
    },
    systems::systems_constants::{HOVERED_BUTTON, NORMAL_BUTTON},
};
use bevy::prelude::*;

/// Select the mission when the button is pressed
///
/// - 1 - We get the ID from the unique id inserted in the node button
/// - 2 - We assign with this ID the selected mission
/// - 3 - We open de details mission modal
pub fn select_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &Mission),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    missions: Res<Missions>,
    mut selected_mission: ResMut<SelectedMission>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
) {
    let mut window = windows.single_mut();
    if !mission_modal_visibility.0 {
        for (interaction, mut color, unique_id, mission) in &mut interaction_query {
            if unique_id.0 == "select_mission_button" {
                match *interaction {
                    Interaction::Pressed => {
                        let mission_id = mission.id;

                        // Search the mission by id in the player_disponible missions
                        selected_mission.mission = missions
                            .0
                            .iter()
                            .find(|mission| mission.id == mission_id)
                            .cloned();

                        mission_modal_visibility.0 = true;
                    }
                    Interaction::Hovered => {
                        window.cursor.icon = CursorIcon::Pointer;
                        *color = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        window.cursor.icon = CursorIcon::Default;
                        *color = NORMAL_BUTTON.into();
                    }
                }
            }
        }
    }
}
