use crate::{
    enums::RoomEnum,
    structs::{
        equipments::ItemEnum,
        general_structs::{
            DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
        },
        player_stats::PlayerStats,
        recruits::SelectedRecruitForMission,
    },
    utils::reset_modals_visibility,
};
use bevy::prelude::*;

pub fn move_room_from_keyboard(
    mut player_stats: ResMut<PlayerStats>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
        );
        player_stats.room = RoomEnum::CommandRoom;
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
        );
        player_stats.room = RoomEnum::Office;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        reset_modals_visibility(
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut daily_events_modal_visibility,
            &mut selected_recruit_for_mission,
        );
        player_stats.room = RoomEnum::Barrack;
    }
}

pub fn delete_item_from_player_inventory(player_stats: &mut PlayerStats, item: &ItemEnum) {
    let item_index = player_stats
        .inventory
        .iter()
        .position(|x| x == item)
        .unwrap();

    player_stats.inventory.remove(item_index);
}
