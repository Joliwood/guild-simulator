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
        (&Interaction, &mut BackgroundColor, &UniqueId, &Mission),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    _missions: Res<Missions>,
    mut selected_mission: ResMut<SelectedMission>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
) {
    let _window = windows.single_mut();
    if !mission_modal_visibility.0 {
        for (interaction, mut background_color, unique_id, mission) in &mut interaction_query {
            // WIP - Change the method to query it
            if unique_id.0 == "select_mission_button" {
                match *interaction {
                    Interaction::Pressed => {
                        selected_mission.mission_id = Some(mission.id);
                        background_color.0 = Color::BLACK;
                        mission_modal_visibility.0 = true;
                    }
                    Interaction::Hovered => {
                        // window.cursor.icon = CursorIcon::Pointer;
                        background_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        // window.cursor.icon = CursorIcon::Default;
                        background_color.0 = Color::BLACK;
                    }
                }
            }
        }
    }
}
