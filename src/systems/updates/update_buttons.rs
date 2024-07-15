use crate::{
    enums::{RecruitEnum, RoomDirectionEnum, RoomEnum},
    structs::{PlayerStats, RecruitStats, SelectRecruitEvent, SelectedRecruit, UniqueId},
    systems::{
        recruits::hire_new_recruits::hire_new_recruits,
        systems_constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    },
    ui::ui_constants::WOOD_COLOR,
    utils::{get_new_room, select_recruit},
};
use bevy::prelude::*;
use uuid::Uuid;

pub fn mouse_interaction_updates(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &UniqueId,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    // Directly filter the interaction query by UniqueId
    for (interaction, mut color, mut border_color, children, unique_id) in &mut interaction_query {
        if unique_id.0 == "menu_button_id" {
            // Safely get the child text component
            if let Ok(mut text) = text_query.get_mut(children[0]) {
                match *interaction {
                    Interaction::Pressed => {
                        // println!("Button pressed");
                        text.sections[0].value = "O".to_string();
                        player_stats.increment_golds(1);
                        *color = PRESSED_BUTTON.into();
                        border_color.0 = Color::srgba(255.0, 0.0, 0.0, 1.0);
                    }
                    Interaction::Hovered => {
                        text.sections[0].value = "H".to_string();
                        *color = HOVERED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                        window.cursor.icon = CursorIcon::Pointer;
                    }
                    Interaction::None => {
                        text.sections[0].value = "X".to_string();
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                        window.cursor.icon = CursorIcon::Grab;
                    }
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
                    window.cursor.icon = CursorIcon::Grab;
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
                    window.cursor.icon = CursorIcon::Grab;
                }
            }
        }

        if unique_id.0 == "room_top_arrow_id" {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Top) {
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
                    window.cursor.icon = CursorIcon::Grab;
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
                    window.cursor.icon = CursorIcon::Grab;
                }
            }
        }

        let new_recruits = vec![RecruitStats {
            id: Uuid::new_v4(),
            class: RecruitEnum::Rogue,
            endurance: 5,
            experience: 0,
            intelligence: 12,
            level: 1,
            max_experience: 100,
            strength: 2,
        }];

        if unique_id.0 == "waz" {
            match *interaction {
                Interaction::Pressed => {
                    println!("let's recruit a rogue now!");
                    hire_new_recruits(player_stats.as_mut(), new_recruits);
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }

        // if unique_id.0 == "recruit_buttons" {
        //     match *interaction {
        //         Interaction::Pressed => {}
        //         Interaction::Hovered => {
        //             window.cursor.icon = CursorIcon::Pointer;
        //             *color = HOVERED_BUTTON.into();
        //         }
        //         Interaction::None => {
        //             window.cursor.icon = CursorIcon::Grab;
        //             *color = BackgroundColor(WOOD_COLOR)
        //         }
        //     }
        // }
    }
}

// pub fn select_recruit_button(
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &Children,
//             &UniqueId,
//         ),
//         Changed<Interaction>,
//     >,
//     mut windows: Query<&mut Window>,
// ) {
//     let mut window = windows.single_mut();

//     for (interaction, mut color, mut border_color, children, unique_id) in &mut interaction_query {
//         if unique_id.0 == "recruit_buttons" {
//             match *interaction {
//                 Interaction::Pressed => {
//                     println!("Button pressed");
//                 }
//                 Interaction::Hovered => {
//                     window.cursor.icon = CursorIcon::Pointer;
//                     *color = HOVERED_BUTTON.into();
//                 }
//                 Interaction::None => {
//                     window.cursor.icon = CursorIcon::Grab;
//                     *color = BackgroundColor(WOOD_COLOR);
//                 }
//             }
//         }
//     }
// }
// WIP
pub fn select_recruit_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &UniqueId,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut event_writer: EventWriter<SelectRecruitEvent>,
    player_stats: Res<PlayerStats>,
    mut selected_recruit: ResMut<SelectedRecruit>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, mut border_color, children, unique_id) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if unique_id.0.starts_with("recruit_button_") {
                    let recruit_id = unique_id.0.strip_prefix("recruit_button_").unwrap();
                    if let Ok(recruit_id) = Uuid::parse_str(recruit_id) {
                        event_writer.send(SelectRecruitEvent(recruit_id));
                    }

                    let recruit_selected = player_stats
                        .recruits
                        .iter()
                        .find(|recruit| recruit.id.to_string() == recruit_id);

                    selected_recruit.0 = recruit_selected.cloned();
                    // select_recruit(selected_recruit, recruit_selected.unwrap().clone());

                    println!(
                        "\n Button pressed on the recruit with the id : {:?}\n",
                        recruit_selected
                    );
                }
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Grab;
                *color = BackgroundColor(WOOD_COLOR);
            }
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
