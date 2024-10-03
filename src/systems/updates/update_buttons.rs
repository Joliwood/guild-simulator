#![allow(unused_mut)]
use crate::{
    audio::play_sound::play_sound,
    enums::{RecruitEnum, RecruitStateEnum, RoomDirectionEnum, RoomEnum, SoundEnum},
    structs::{
        equipments::Item,
        general_structs::{
            load_scroll_by_id, MissionModalVisible, Missions, PlayerStats, RecruitInventory,
            RecruitStats, SelectedMission, SelectedRecruit, UniqueId,
        },
    },
    systems::{
        recruits::hire_new_recruits::hire_new_recruits,
        systems_constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    },
    ui::ui_constants::WOOD_COLOR,
    utils::{
        equip_recruit_inventory, get_global_points, get_new_room, get_victory_percentage,
        get_xp_earned, is_mission_success,
    },
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
    mut modal_visible: ResMut<MissionModalVisible>,
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
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Right) {
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
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Left) {
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
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Top) {
                        player_stats.room = new_room;
                    }
                    modal_visible.0 = false;
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
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Bottom) {
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
                    let new_item = load_scroll_by_id(2);
                    if let Some(item) = new_item {
                        player_stats.add_item(Item::Scroll(item, 1));
                    }
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }
}

/// Select the recruit when the button is pressed
pub fn select_recruit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit: ResMut<SelectedRecruit>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, recruit) in &mut interaction_query {
        if unique_id.0 == "recruit_button" {
            match *interaction {
                Interaction::Pressed => {
                    selected_recruit.0 = Some(recruit.clone());
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

/// Select the mission when the button is pressed
///
/// - 1 - We get the ID from the unique id inserted in the node button
/// - 2 - We assign with this ID the selected mission
/// - 3 - We open de details mission modal
pub fn select_mission_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    missions: Res<Missions>,
    mut selected_mission: ResMut<SelectedMission>,
    mut modal_visible: ResMut<MissionModalVisible>,
) {
    let mut window = windows.single_mut();
    if !modal_visible.0 {
        for (interaction, mut color, unique_id) in &mut interaction_query {
            if unique_id.0.starts_with("select_mission_button_") {
                match *interaction {
                    Interaction::Pressed => {
                        let mission_id =
                            unique_id.0.strip_prefix("select_mission_button_").unwrap();

                        // Search the mission by id in the player_disponible missions
                        selected_mission.mission = missions
                            .0
                            .iter()
                            .find(|mission| mission.id.to_string() == mission_id)
                            .cloned();

                        modal_visible.0 = true;
                    }
                    Interaction::Hovered => {
                        window.cursor.icon = CursorIcon::Pointer;
                        *color = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        window.cursor.icon = CursorIcon::Default;
                        *color = NORMAL_BUTTON.into();
                    }
                }
            }
        }
    }
}

/// On arrive ici en cliquant dans la modal mission sur la recruit
///
/// - 1 - On retrouve l'id de la recruit directement
/// - 2 - On vient modifier la selected mission pour lui assigner l'id de la recruit
/// - 3 - On va chercher la recrue avec son id fourni dans la selected mission
/// - 4 - On calcule le score global de la recrue
/// - 5 - On calcule le score global de l'ennemi de la selected mission
/// - 6 - On calcule le % de victoire
/// - 7 - On update la selected mission avec le % de victoire
pub fn assign_recruit_to_mission(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    player_stats: Res<PlayerStats>,
    mut selected_mission: ResMut<SelectedMission>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id) in &mut interaction_query {
        if unique_id.0.starts_with("assign_recruit_to_mission_") {
            match *interaction {
                Interaction::Pressed => {
                    // - 1 - //
                    let recruit_id = unique_id
                        .0
                        .strip_prefix("assign_recruit_to_mission_")
                        .unwrap();

                    // - 2 - //
                    selected_mission.recruit_id = Some(Uuid::parse_str(recruit_id).unwrap());

                    // - 3 - //
                    let recruit_selected = player_stats
                        .recruits
                        .iter()
                        .find(|recruit| recruit.id.to_string() == recruit_id);

                    // - 4 - //
                    let recruit_global_points = get_global_points(
                        recruit_selected.unwrap().strength,
                        recruit_selected.unwrap().endurance,
                        recruit_selected.unwrap().intelligence,
                    );

                    // - 5 - //
                    let ennemy = &selected_mission.mission.as_ref().unwrap().ennemy;
                    let ennemy_global_points =
                        get_global_points(ennemy.strength, ennemy.endurance, ennemy.intelligence);

                    // - 6 - //
                    let victory_percentage =
                        get_victory_percentage(recruit_global_points, ennemy_global_points);

                    let victory_percentage_rounded: u32 = victory_percentage.round() as u32;

                    // - 7 - //
                    selected_mission.percent_of_victory = Some(victory_percentage_rounded);
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
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, mut border_color) in &mut interaction_query {
        if unique_id.0.starts_with("close_mission_modal") {
            match *interaction {
                Interaction::Pressed => {
                    modal_visible.0 = false;
                    border_color.0 = WOOD_COLOR;
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
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
    mut selected_mission: ResMut<SelectedMission>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id) in &mut interaction_query {
        // TODO - Start the mission with provided id of mission + recruit (not disponible)
        if selected_mission.recruit_id != None {
            if unique_id.0.starts_with("start_mission") {
                match *interaction {
                    Interaction::Pressed => {
                        info!(
                            "% of win is : {:?}",
                            selected_mission.percent_of_victory.as_ref().unwrap()
                        );
                        let percent_of_victory =
                            selected_mission.percent_of_victory.unwrap() as f32;
                        let is_mission_sucess = is_mission_success(percent_of_victory);
                        if is_mission_sucess {
                            info!("The mission is a success !",);
                            let mission_ennemy_level =
                                selected_mission.mission.as_ref().unwrap().level;
                            let xp_earned = get_xp_earned(mission_ennemy_level);
                            let gold_earned = (mission_ennemy_level * 10) as i32;
                            let recruit_id = selected_mission.recruit_id.unwrap();

                            player_stats.gain_xp_to_recruit(recruit_id, xp_earned);
                            player_stats.increment_golds(gold_earned);
                        } else {
                            info!("The mission is a failure !");
                        }
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
}

pub fn move_room_from_keyboard(
    mut player_stats: ResMut<PlayerStats>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        info!("Right arrow pressed");
        if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Right) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Left) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Top) {
            player_stats.room = new_room;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Bottom) {
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
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &UniqueId,
            &mut BorderColor,
            &Item,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit: ResMut<SelectedRecruit>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, mut border_color, item) in &mut interaction_query {
        if unique_id.0 == "item_in_inventory" {
            match *interaction {
                Interaction::Pressed => {
                    border_color.0 = WOOD_COLOR;
                    let is_recruit_equiped =
                        equip_recruit_inventory(&mut selected_recruit, item, &mut player_stats);
                    if is_recruit_equiped == true {
                        match item {
                            Item::Armor(_) => {
                                play_sound(&asset_server, &mut commands, SoundEnum::EquipArmor);
                            }
                            Item::Scroll(_, __) => {
                                play_sound(&asset_server, &mut commands, SoundEnum::EquipScroll);
                            }
                            Item::Weapon(_) => {
                                play_sound(&asset_server, &mut commands, SoundEnum::EquipWeapon);
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

pub fn delete_item_from_player_inventory(player_stats: &mut PlayerStats, item: &Item) {
    let item_index = player_stats
        .inventory
        .iter()
        .position(|x| x == item)
        .unwrap();

    player_stats.inventory.remove(item_index);
}
