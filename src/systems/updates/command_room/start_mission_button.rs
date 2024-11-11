use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::{SelectedRecruitForEquipment, SelectedRecruitForMission},
    },
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn start_mission_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor, &UniqueId), Changed<Interaction>>,
    mut missions: ResMut<Missions>,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    mut selected_mission: ResMut<SelectedMission>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
) {
    let _window = windows.single_mut();

    for (interaction, mut border_color, unique_id) in &mut interaction_query {
        if selected_mission.recruit_id.is_some()
            && selected_mission.mission_id.is_some()
            && unique_id.0 == "start_mission"
        {
            match *interaction {
                Interaction::Pressed => {
                    let recruit_id = selected_mission.recruit_id.unwrap();
                    let selected_mission_id = selected_mission.mission_id.unwrap();

                    player_stats.update_state_of_recruit(recruit_id, RecruitStateEnum::InMission);

                    missions.assign_recruit_id_to_mission_by_id(selected_mission_id, recruit_id);

                    missions.attribute_days_left_by_mission_id(selected_mission_id);

                    // To avoid changing equipments of a recruit sent to a mission (and falsify % win),
                    // we de-select the recruit for equipment if it's the same send to the mission
                    if selected_recruit_for_equipment.0.is_some()
                        && selected_recruit_for_equipment.0.clone().unwrap().id == recruit_id
                    {
                        selected_recruit_for_equipment.0 = None;
                    }

                    mission_modal_visibility.0 = false;
                    selected_mission.reset();
                    selected_recruit_for_mission.0 = None;
                }
                Interaction::Hovered => {
                    // window.cursor.icon = CursorIcon::Pointer;
                    *border_color = BorderColor(Color::WHITE);
                }
                Interaction::None => {
                    // window.cursor.icon = CursorIcon::Default;
                    *border_color = BorderColor(Color::BLACK);
                }
            }
        }
    }
}
