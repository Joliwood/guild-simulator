use crate::structs::{
    general_structs::MissionReportsModalVisible, missions::MissionReports,
    trigger_structs::MissionReport,
};
use bevy::prelude::*;

pub fn toggle_mission_reports(
    mut query: Query<&Interaction, (Changed<Interaction>, With<MissionReport>)>,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mission_reports: Res<MissionReports>,
) {
    let mut window = windows.single_mut();

    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let mission_reports_number = mission_reports.0.len();
                if mission_reports_number > 0 {
                    mission_reports_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
