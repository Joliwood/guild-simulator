use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    structs::{
        general_structs::MissionReportsModalVisible,
        missions::{MissionReport, MissionReports, Missions},
        player_stats::PlayerStats,
        trigger_structs::MissionReportModalSignButtonTrigger,
    },
    systems::systems_constants::HOVERED_BUTTON,
    utils::is_mission_success,
};
use bevy::prelude::*;

pub fn sign_mission_report(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &MissionReportModalSignButtonTrigger,
            &mut BackgroundColor,
            &MissionReport,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut player_stats: ResMut<PlayerStats>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut style, _button, mut color, mission_report) in interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                player_stats.update_state_of_recruit(
                    mission_report.recruit_id,
                    RecruitStateEnum::Available,
                );

                missions.desassign_recruit_to_mission(mission_report.recruit_id);

                let is_mission_sucess =
                    is_mission_success(mission_report.percent_of_victory as f32);

                if is_mission_sucess {
                    player_stats.gain_xp(mission_report.experience_gained.unwrap());
                    player_stats.gain_xp_to_recruit(
                        mission_report.recruit_id,
                        mission_report.experience_gained.unwrap(),
                    );
                    player_stats.increment_golds(mission_report.golds_gained.unwrap());
                }

                mission_reports.remove_mission_report_by_id(mission_report.mission_id);

                mission_reports_modal_visibility.0 = false;
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                *color = HOVERED_BUTTON.into();
                // Add a border when hovered
                style.border = UiRect {
                    left: Val::Px(3.0),
                    right: Val::Px(3.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(3.0),
                };
            }
            Interaction::None => {
                // Remove the border when not interacted with
                window.cursor.icon = CursorIcon::Default;
                *color = BackgroundColor(ColorPaletteEnum::DarkBrown.as_color());
                style.border = UiRect::default();
            }
        }
    }
}
