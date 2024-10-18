use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
        player_stats::{self, PlayerStats},
        recruits::{RecruitStats, SelectedRecruitForMission},
    },
    systems::systems_constants::HOVERED_BUTTON,
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

/// Select the recruit when the button is pressed
pub fn select_recruit_for_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut selected_mission: ResMut<SelectedMission>,
    player_stats: Res<PlayerStats>,
    mut missions: ResMut<Missions>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, recruit) in &mut interaction_query {
        let recruit_state = recruit.clone().state;
        if unique_id.0 == "map_recruit_button"
            && recruit_state != RecruitStateEnum::InMission
            && recruit_state != RecruitStateEnum::WaitingReportSignature
        {
            match *interaction {
                Interaction::Pressed => {
                    selected_recruit_for_mission.0 = Some(recruit.clone());
                    // selected_mission.recruit_id = Some(recruit.id);
                    let recruit_id = recruit.id;

                    selected_mission.recruit_id = Some(recruit_id);

                    // ! WIP to check after missions refacto
                    // let recruit_selected = player_stats
                    //     .recruits
                    //     .iter()
                    //     .find(|recruit| recruit.id == recruit_id);

                    // if recruit_selected.is_none() {
                    //     return;
                    // }

                    // let recruit_global_points = recruit_selected.unwrap().get_total_merged_stats();

                    // let ennemy = &selected_mission.mission.as_ref().unwrap().ennemy;
                    // let ennemy_global_points =
                    //     get_global_points(ennemy.strength, ennemy.endurance, ennemy.intelligence);

                    // let victory_percentage =
                    //     get_victory_percentage(recruit_global_points as u16, ennemy_global_points);

                    // let victory_percentage_rounded: u32 = victory_percentage.round() as u32;

                    // selected_mission.percent_of_victory = Some(victory_percentage_rounded);

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
