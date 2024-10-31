use bevy::prelude::*;

use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::{SelectedRecruitForEquipment, SelectedRecruitForMission},
    },
    systems::systems_constants::HOVERED_BUTTON,
    ui::ui_constants::WOOD_COLOR,
};

#[allow(clippy::too_many_arguments)]
pub fn start_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId),
        Changed<Interaction>,
    >,
    mut missions: ResMut<Missions>,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    mut selected_mission: ResMut<SelectedMission>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut modal_visible: ResMut<MissionModalVisible>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id) in &mut interaction_query {
        // TODO - Start the mission with provided id of mission + recruit (not disponible)
        if selected_mission.recruit_id.is_some() && unique_id.0 == "start_mission" {
            match *interaction {
                Interaction::Pressed => {
                    info!(
                        "% of win is : {:?}",
                        selected_mission.percent_of_victory.as_ref().unwrap()
                    );

                    let recruit_id = selected_mission.recruit_id;
                    if recruit_id.is_none() {
                        return;
                    }

                    player_stats
                        .update_state_of_recruit(recruit_id.unwrap(), RecruitStateEnum::InMission);

                    let mission = selected_mission.get_mission();

                    if mission.is_none() {
                        return;
                    }

                    missions.assign_recruit_id_to_mission(
                        mission.clone().unwrap().id,
                        recruit_id.unwrap(),
                    );

                    missions.attribute_days_left_to_mission(mission.unwrap().id);

                    player_stats
                        .update_state_of_recruit(recruit_id.unwrap(), RecruitStateEnum::InMission);

                    let mission = selected_mission.get_mission();

                    if mission.is_none() {
                        return;
                    }

                    if selected_recruit_for_equipment.0.is_some()
                        && selected_recruit_for_equipment.0.clone().unwrap().id
                            == recruit_id.unwrap()
                    {
                        selected_recruit_for_equipment.0 = None;
                    }

                    missions.assign_recruit_id_to_mission(
                        mission.clone().unwrap().id,
                        recruit_id.unwrap(),
                    );

                    missions.attribute_days_left_to_mission(mission.unwrap().id);

                    modal_visible.0 = false;
                    selected_mission.reset();
                    selected_recruit_for_mission.0 = None;
                }
                Interaction::Hovered => {
                    window.cursor.icon = CursorIcon::Pointer;
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    window.cursor.icon = CursorIcon::Default;
                    *color = BackgroundColor(WOOD_COLOR);
                }
            }
        }
    }
}
