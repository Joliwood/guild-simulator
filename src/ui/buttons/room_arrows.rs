use bevy::prelude::*;

use crate::{structs::UniqueId, systems::systems_constants::NORMAL_BUTTON};

pub fn room_left_arrow_button(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                overflow: Overflow {
                    x: OverflowAxis::Visible,
                    y: OverflowAxis::Visible,
                },
                left: Val::Px(0.0),
                display: Display::Flex,
                margin: UiRect::all(Val::Auto),
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            ..default()
        })
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    // border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_left_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "<",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_right_arrow_button(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                display: Display::Flex,
                margin: UiRect::all(Val::Auto),
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            ..default()
        })
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_right_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        ">",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_bottom_arrow_button(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                display: Display::Flex,
                margin: UiRect::all(Val::Auto),
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            ..default()
        })
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_bottom_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "V",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_top_arrow_button(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                display: Display::Flex,
                margin: UiRect::all(Val::Auto),
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                ..default()
            },
            ..default()
        })
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_top_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "^",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
