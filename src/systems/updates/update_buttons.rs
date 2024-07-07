use crate::{
    enums::{RoomDirectionEnum, RoomEnum},
    structs::{PlayerStats, UniqueId},
    systems::systems_constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    utils::get_new_room,
};
use bevy::prelude::*;

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
) -> () {
    let mut window = windows.single_mut();

    // Directly filter the interaction query by UniqueId
    for (interaction, mut color, mut border_color, children, unique_id) in &mut interaction_query {
        if unique_id.0 == "menu_button_id" {
            // Safely get the child text component
            if let Ok(mut text) = text_query.get_mut(children[0]) {
                match *interaction {
                    Interaction::Pressed => {
                        println!("Button pressed");
                        text.sections[0].value = "O".to_string();
                        player_stats.increment_golds(1);
                        *color = PRESSED_BUTTON.into();
                        border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                    }
                    Interaction::Hovered => {
                        text.sections[0].value = "H".to_string();
                        *color = HOVERED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                        window.cursor.icon = CursorIcon::Grab;
                    }
                    Interaction::None => {
                        text.sections[0].value = "X".to_string();
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                        window.cursor.icon = CursorIcon::Pointer;
                    }
                }
            }
        }

        if unique_id.0 == "room_right_arrow_id" {
            // Safely get the child text component
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Right) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Grab;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
            }
        }

        if unique_id.0 == "room_left_arrow_id" {
            // Safely get the child text component
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Left) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Grab;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
            }
        }

        if unique_id.0 == "room_top_arrow_id" {
            // Safely get the child text component
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Top) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Grab;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
            }
        }

        if unique_id.0 == "room_bottom_arrow_id" {
            // Safely get the child text component
            match *interaction {
                Interaction::Pressed => {
                    if let Some(new_room) = get_new_room(&player_stats, RoomDirectionEnum::Bottom) {
                        player_stats.room = new_room;
                    }
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                    *color = HOVERED_BUTTON.into();
                    window.cursor.icon = CursorIcon::Grab;
                }
                Interaction::None => {
                    border_color.0 = Color::BLACK;
                    *color = NORMAL_BUTTON.into();
                    window.cursor.icon = CursorIcon::Pointer;
                }
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
) -> () {
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
