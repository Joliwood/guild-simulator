use crate::{
    audio::play_sound::play_sound,
    content::equipments::scrolls::ScrollsEnum,
    enums::{RecruitEnum, RecruitStateEnum, RoomDirectionEnum, RoomEnum, SoundEnum},
    my_assets::MyAssets,
    structs::{
        equipments::ItemEnum,
        general_structs::{
            load_scroll, DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
            UniqueId,
        },
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::{
            RecruitInventory, RecruitStats, SelectedRecruitForEquipment, SelectedRecruitForMission,
        },
    },
    systems::{
        recruits::hire_new_recruits::hire_new_recruits,
        systems_constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    },
    ui::ui_constants::WOOD_COLOR,
    utils::{equip_recruit_inventory, get_new_room},
};
use bevy::prelude::*;
use uuid::Uuid;

pub fn mouse_interaction_updates(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &UniqueId,
        ),
        Changed<Interaction>,
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    let mut window = windows.single_mut();

    // Directly filter the interaction query by UniqueId
    for (interaction, mut color, mut border_color, unique_id) in &mut interaction_query {
        if unique_id.0 == "menu_button_id" {
            // Safely get the child text component
            match *interaction {
                Interaction::Pressed => {
                    player_stats.increment_golds(1);
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = WOOD_COLOR;
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                    window.cursor.icon = CursorIcon::Pointer;
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                    window.cursor.icon = CursorIcon::Default;
                }
            }
        }

        if unique_id.0 == "room_right_arrow_id" {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(
                        &player_stats,
                        RoomDirectionEnum::Right,
                        &mut mission_modal_visibility,
                        &mut mission_reports_modal_visibility,
                        &mut selected_recruit_for_mission,
                        &mut daily_events_modal_visibility,
                    ) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 1.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Default;
                }
            }
        }

        if unique_id.0 == "room_left_arrow_id" {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(
                        &player_stats,
                        RoomDirectionEnum::Left,
                        &mut mission_modal_visibility,
                        &mut mission_reports_modal_visibility,
                        &mut selected_recruit_for_mission,
                        &mut daily_events_modal_visibility,
                    ) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 1.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Default;
                }
            }
        }

        if unique_id.0 == "room_top_arrow_id" {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(
                        &player_stats,
                        RoomDirectionEnum::Top,
                        &mut mission_modal_visibility,
                        &mut mission_reports_modal_visibility,
                        &mut selected_recruit_for_mission,
                        &mut daily_events_modal_visibility,
                    ) {
                        player_stats.room = new_room;
                    }
                    // mission_modal_visibility.0 = false;
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 1.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Default;
                }
            }
        }

        if unique_id.0 == "room_bottom_arrow_id" {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(
                        &player_stats,
                        RoomDirectionEnum::Bottom,
                        &mut mission_modal_visibility,
                        &mut mission_reports_modal_visibility,
                        &mut selected_recruit_for_mission,
                        &mut daily_events_modal_visibility,
                    ) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 1.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Default;
                }
            }
        }

        let new_recruits = vec![RecruitStats {
            class: RecruitEnum::Rogue,
            endurance: 5,
            experience: 0,
            id: Uuid::new_v4(),
            image_atlas_index: 3,
            intelligence: 12,
            level: 1,
            max_experience: 100,
            name: "Big Noob".to_string(),
            recruit_inventory: RecruitInventory::generate_empty_inventory(),
            state: RecruitStateEnum::Available,
            strength: 2,
        }];

        if unique_id.0 == "waz" {
            match *interaction {
                Interaction::Pressed => {
                    info!("let's recruit a rogue now!");
                    hire_new_recruits(player_stats.as_mut(), new_recruits);
                    let new_item = load_scroll(ScrollsEnum::ScrollOfPower);
                    // if let Some(item) = new_item {
                    player_stats.add_item(ItemEnum::Scroll(new_item, 1));
                    // }
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }
}

pub fn move_room_from_keyboard(
    mut player_stats: ResMut<PlayerStats>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        info!("Right arrow pressed");
        if let Some(new_room) = get_new_room(
            &player_stats,
            RoomDirectionEnum::Right,
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut daily_events_modal_visibility,
        ) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        if let Some(new_room) = get_new_room(
            &player_stats,
            RoomDirectionEnum::Left,
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut daily_events_modal_visibility,
        ) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        if let Some(new_room) = get_new_room(
            &player_stats,
            RoomDirectionEnum::Top,
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut daily_events_modal_visibility,
        ) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        if let Some(new_room) = get_new_room(
            &player_stats,
            RoomDirectionEnum::Bottom,
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
            &mut selected_recruit_for_mission,
            &mut daily_events_modal_visibility,
        ) {
            player_stats.room = new_room;
        }
    }
}

/// Disable the buttons that are not needed in the current room
///
/// # Parameters
/// - `player_stats`: Check of the player room
/// - `query`: What we will change (first arg) in a specific UniqId (second arg)
pub fn buttons_disable_updates(
    player_stats: Res<PlayerStats>,
    mut query: Query<(&mut Style, &UniqueId)>,
) {
    for (mut style, unique_id) in query.iter_mut() {
        match player_stats.room {
            RoomEnum::Office => match unique_id.0.as_str() {
                "room_top_arrow_id" => {
                    style.display = Display::None;
                }
                // WIP - Desactivated for a V0
                "room_left_arrow_id" => {
                    style.display = Display::None;
                }
                _ => style.display = Display::Flex,
            },
            RoomEnum::Barrack => match unique_id.0.as_str() {
                "room_right_arrow_id" => {
                    style.display = Display::None;
                }
                "room_top_arrow_id" => {
                    style.display = Display::None;
                }
                "room_bottom_arrow_id" => {
                    style.display = Display::None;
                }
                _ => style.display = Display::Flex,
            },
            RoomEnum::Store => match unique_id.0.as_str() {
                "room_left_arrow_id" => {
                    style.display = Display::None;
                }
                "room_top_arrow_id" => {
                    style.display = Display::None;
                }
                "room_bottom_arrow_id" => {
                    style.display = Display::None;
                }
                _ => style.display = Display::Flex,
            },
            RoomEnum::CommandRoom => match unique_id.0.as_str() {
                "room_right_arrow_id" => {
                    style.display = Display::None;
                }
                "room_left_arrow_id" => {
                    style.display = Display::None;
                }
                "room_bottom_arrow_id" => {
                    style.display = Display::None;
                }
                _ => style.display = Display::Flex,
            },
        }
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
