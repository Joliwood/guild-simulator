use crate::{
    audio::play_sound::play_sound,
    data::equipments::scrolls::ScrollsEnum,
    enums::{RecruitEnum, RecruitStateEnum, RoomDirectionEnum, RoomEnum, SoundEnum},
    structs::{
        equipments::ItemEnum,
        general_structs::{load_scroll, MissionModalVisible, MissionReportsModalVisible, UniqueId},
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
    ui::{interface::gold_counter::MyAssets, ui_constants::WOOD_COLOR},
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
            name: "Random noob".to_string(),
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

pub fn close_mission_modal(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &UniqueId,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut modal_visible: ResMut<MissionModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, mut border_color) in &mut interaction_query {
        if unique_id.0 == "close_mission_modal" || unique_id.0 == "start_mission" {
            match *interaction {
                Interaction::Pressed => {
                    modal_visible.0 = false;
                    border_color.0 = WOOD_COLOR;

                    if selected_recruit_for_equipment.0 == selected_recruit_for_mission.0 {
                        selected_recruit_for_equipment.0 = None;
                    }

                    selected_recruit_for_mission.0 = None;
                }
                Interaction::Hovered => {
                    window.cursor.icon = CursorIcon::Pointer;
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    window.cursor.icon = CursorIcon::Default;
                    *color = BackgroundColor(WOOD_COLOR);
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}

pub fn start_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId),
        Changed<Interaction>,
    >,
    mut missions: ResMut<Missions>,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    selected_mission: ResMut<SelectedMission>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id) in &mut interaction_query {
        // TODO - Start the mission with provided id of mission + recruit (not disponible)
        if selected_mission.recruit_id.is_some() && unique_id.0 == "start_mission" {
            match *interaction {
                Interaction::Pressed => {
                    info!(
                        "% of win is : {:?}",
                        selected_mission.percent_of_victory.as_ref().unwrap()
                    );

                    let recruit_id = selected_mission.recruit_id;
                    if recruit_id.is_none() {
                        return;
                    }

                    player_stats
                        .update_state_of_recruit(recruit_id.unwrap(), RecruitStateEnum::InMission);

                    let mission = selected_mission.get_mission();

                    if mission.is_none() {
                        return;
                    }

                    missions.assign_recruit_id_to_mission(
                        mission.clone().unwrap().id,
                        recruit_id.unwrap(),
                    );

                    missions.attribute_days_left_to_mission(mission.unwrap().id);

                    player_stats
                        .update_state_of_recruit(recruit_id.unwrap(), RecruitStateEnum::InMission);

                    let mission = selected_mission.get_mission();

                    if mission.is_none() {
                        return;
                    }

                    missions.assign_recruit_id_to_mission(
                        mission.clone().unwrap().id,
                        recruit_id.unwrap(),
                    );

                    missions.attribute_days_left_to_mission(mission.unwrap().id);
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

pub fn move_room_from_keyboard(
    mut player_stats: ResMut<PlayerStats>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        info!("Right arrow pressed");
        if let Some(new_room) = get_new_room(
            &player_stats,
            RoomDirectionEnum::Right,
            &mut mission_modal_visibility,
            &mut mission_reports_modal_visibility,
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

pub fn select_item_in_inventory(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &UniqueId,
            &mut BorderColor,
            &ItemEnum,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, mut border_color, item) in &mut interaction_query {
        if unique_id.0 == "item_in_inventory" {
            match *interaction {
                Interaction::Pressed => {
                    border_color.0 = WOOD_COLOR;
                    let is_recruit_equiped = equip_recruit_inventory(
                        &mut selected_recruit_for_equipment,
                        item,
                        &mut player_stats,
                    );
                    if is_recruit_equiped {
                        match item {
                            ItemEnum::Armor(_) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipArmor);
                            }
                            ItemEnum::Scroll(_, _) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipScroll);
                            }
                            ItemEnum::Weapon(_) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipWeapon);
                            }
                        }
                    }
                }
                Interaction::Hovered => {
                    window.cursor.icon = CursorIcon::Pointer;
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    window.cursor.icon = CursorIcon::Default;
                    *color = BackgroundColor(WOOD_COLOR);
                    border_color.0 = Color::BLACK;
                }
            }
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
