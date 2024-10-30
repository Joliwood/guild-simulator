use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::RecruitStats,
    },
    systems::systems_constants::HOVERED_BUTTON,
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

/// On arrive ici en cliquant dans la modal mission sur la recruit
pub fn assign_recruit_to_mission(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    player_stats: Res<PlayerStats>,
    mut selected_mission: ResMut<SelectedMission>,
    mut missions: ResMut<Missions>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, recruit) in &mut interaction_query {
        if unique_id.0 == "assign_recruit_to_mission"
            && recruit.state != RecruitStateEnum::InMission
            && recruit.state != RecruitStateEnum::WaitingReportSignature
        {
            match *interaction {
                Interaction::Pressed => {
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
