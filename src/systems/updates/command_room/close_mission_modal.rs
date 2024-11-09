use crate::{
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::SelectedMission,
        recruits::SelectedRecruitForMission,
    },
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

pub fn close_mission_modal(
    mut interaction_query: Query<(&Interaction, &UniqueId, &mut BorderColor), Changed<Interaction>>,
    _windows: Query<&mut Window>,
    mut modal_visible: ResMut<MissionModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut selected_mission: ResMut<SelectedMission>,
) {
    // let mut window = windows.single_mut();

    for (interaction, unique_id, mut border_color) in &mut interaction_query {
        if unique_id.0 == "close_mission_modal" {
            match *interaction {
                Interaction::Pressed => {
                    modal_visible.0 = false;
                    border_color.0 = WOOD_COLOR;
                    selected_recruit_for_mission.0 = None;
                    selected_mission.reset();
                }
                Interaction::Hovered => {
                    // window.cursor.icon = CursorIcon::Pointer;
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    // window.cursor.icon = CursorIcon::Default;
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}
