use crate::{
    structs::{
        general_structs::{
            DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
            TutoMessagesModalVisible,
        },
        recruits::SelectedRecruitForMission,
    },
    utils::reset_modals_visibility,
};
use bevy::prelude::*;

pub fn close_modal_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
    mut tuto_messages_modal_visibility: ResMut<TutoMessagesModalVisible>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut tuto_messages_modal_visibility,
        );
    }
}
