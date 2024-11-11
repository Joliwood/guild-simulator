use crate::{
    enums::RoomEnum,
    structs::{
        general_structs::{
            DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
        },
        player_stats::PlayerStats,
        recruits::SelectedRecruitForMission,
        trigger_structs::RoomButtonTrigger,
    },
    utils::reset_modals_visibility,
};
use bevy::{
    prelude::*,
    //  window::CursorOptions
};

// System to update room based on button clicks
pub fn update_room_on_click(
    mut interaction_query: Query<
        (&Interaction, &RoomButtonTrigger, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    let _window = windows.single_mut();

    for (interaction, room_button_trigger, mut _background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
                // background_color.0 = Color::srgba(0., 0., 0., 0.6);
            }
            Interaction::Pressed => {
                reset_modals_visibility(
                    &mut mission_modal_visibility,
                    &mut mission_reports_modal_visibility,
                    &mut daily_events_modal_visibility,
                    &mut selected_recruit_for_mission,
                );
                match room_button_trigger.0 {
                    RoomEnum::CommandRoom => player_stats.room = RoomEnum::CommandRoom,
                    RoomEnum::Office => player_stats.room = RoomEnum::Office,
                    RoomEnum::Barrack => player_stats.room = RoomEnum::Barrack,
                    _ => (),
                }
            }
            Interaction::None => {
                // background_color.0 = Color::NONE;
                // window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
