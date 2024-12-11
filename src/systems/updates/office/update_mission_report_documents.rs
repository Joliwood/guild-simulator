use crate::structs::{
    daily_events_folder::daily_events::DailyEvents,
    missions::MissionReports,
    trigger_structs::{DailyEventTextTrigger, DailyEventTrigger, MissionReportTrigger},
};
use bevy::prelude::*;

// WIP - Faire le text

pub fn update_mission_report_documents(
    mission_reports: Res<MissionReports>,
    mut query: Query<(&mut Node, &mut MissionReportTrigger)>,
    // text_query: Single<Entity, (With<DailyEventTextTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let mission_reports_number = mission_reports.0.len();
    info!("Mission reports number: {}", mission_reports_number);

    for (mut node, _) in query.iter_mut() {
        if mission_reports_number > 0 {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }

    // *writer.text(*text_query, 0) = format!("{}", daily_events_number);
}
