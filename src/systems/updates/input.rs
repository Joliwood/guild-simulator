use crate::{
    enums::RoomEnum,
    structs::{
        general_structs::{
            DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
            NotificationCount, TutoMessagesModalVisible,
        },
        player_stats::{PlayerStats, TutoEnum, TutoMessages},
        recruits::SelectedRecruitForMission,
    },
    utils::reset_modals_visibility,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn move_room_from_keyboard(
    mut player_stats: ResMut<PlayerStats>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
    mut notification_count: ResMut<NotificationCount>,
    mut tuto_messages_modal_visibility: ResMut<TutoMessagesModalVisible>,
    mut tuto_messages: ResMut<TutoMessages>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut tuto_messages_modal_visibility,
        );
        player_stats.room = RoomEnum::CommandRoom;
        if player_stats.tuto.is_command_room_tuto_done.is_none() {
            player_stats.tuto.is_command_room_tuto_done = Some(false);
            tuto_messages.add_tuto_message(TutoEnum::CommandRoom);
        }
        notification_count.command_room_count = 0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut tuto_messages_modal_visibility,
        );
        player_stats.room = RoomEnum::Office;
        notification_count.office_count = 0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut tuto_messages_modal_visibility,
        );
        player_stats.room = RoomEnum::Barrack;
        if player_stats.tuto.is_barrack_room_tuto_done.is_none() {
            player_stats.tuto.is_barrack_room_tuto_done = Some(false);
            tuto_messages.add_tuto_message(TutoEnum::BarrackRoom);
        }
        notification_count.barrack_count = 0;
    }
}
