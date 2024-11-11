use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::{RecruitStats, SelectedRecruitForMission},
    },
};
use bevy::prelude::*;

pub fn select_recruit_for_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut selected_mission: ResMut<SelectedMission>,
    player_stats: Res<PlayerStats>,
    mut missions: ResMut<Missions>,
) {
    let _window = windows.single_mut();

    for (interaction, mut border_color, unique_id, recruit) in &mut interaction_query {
        let recruit_state = recruit.clone().state;
        if unique_id.0 == "map_recruit_button"
            && recruit_state != RecruitStateEnum::InMission
            && recruit_state != RecruitStateEnum::WaitingReportSignature
        {
            match *interaction {
                Interaction::Pressed => {
                    // Ensure we can assign a recruit to a mission only if a mission is selected
                    if let Some(mission_id) = selected_mission.mission_id {
                        // Assign the recruit to the mission order
                        selected_recruit_for_mission.0 = Some(recruit.clone());

                        // Assign the recruit to the mission
                        selected_mission.recruit_id = Some(recruit.id);

                        // Find the mission we are talking about
                        let mission = match missions.get_mission_by_id(&mission_id) {
                            Some(mission) => mission,
                            None => {
                                error!(
                                    "The mission doesn't exist for this mission_id : {}",
                                    mission_id
                                );
                                return;
                            }
                        };

                        // Calculate the victory % for selected mission only
                        selected_mission.calculate_percent_of_victory(&mission, &player_stats);

                        // Assign the victory % to the real mission
                        // It's the report doc sign that will calculate if it's win / lose
                        match selected_mission.percent_of_victory {
                            Some(victory_percentage) => {
                                missions.attribute_percent_of_victory_to_mission(
                                    mission.id,
                                    victory_percentage,
                                );
                            }
                            None => {
                                error!(
                                    "The victory percentage should already been set, for this mission_id : {}",
                                    mission_id
                                );
                            }
                        }
                    }
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
