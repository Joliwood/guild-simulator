use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::{RecruitStats, SelectedRecruitForMission},
    },
    systems::systems_constants::HOVERED_BUTTON,
};
use bevy::prelude::*;

/// Select the recruit when the button is pressed
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

    for (interaction, mut color, unique_id, recruit) in &mut interaction_query {
        let recruit_state = recruit.clone().state;
        if unique_id.0 == "map_recruit_button"
            && recruit_state != RecruitStateEnum::InMission
            && recruit_state != RecruitStateEnum::WaitingReportSignature
        {
            match *interaction {
                Interaction::Pressed => {
                    if selected_mission.mission.is_none() {
                        return;
                    }

                    selected_recruit_for_mission.0 = Some(recruit.clone());
                    let recruit_id = recruit.id;

                    selected_mission.recruit_id = Some(recruit_id);

                    selected_mission.calculate_percent_of_victory(&player_stats);
                    let victory_percentage_rounded = selected_mission.percent_of_victory.unwrap();

                    let mission = selected_mission.mission.as_ref();
                    if mission.is_none() {
                        return;
                    }

                    missions.attribute_percent_of_victory_to_mission(
                        mission.unwrap().id,
                        victory_percentage_rounded,
                    );
                }
                Interaction::Hovered => {
                    // window.cursor.icon = CursorIcon::Pointer;
                    *color = BorderColor(Color::WHITE);
                }
                Interaction::None => {
                    // window.cursor.icon = CursorIcon::Default;
                    *color = HOVERED_BUTTON.into();
                }
            }
        }
    }
}
