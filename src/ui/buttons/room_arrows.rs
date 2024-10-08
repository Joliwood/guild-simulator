use crate::{
    custom_components::CustomButton,
    structs::general_structs::UniqueId,
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::room_arrow_button_style},
};
use bevy::prelude::*;

pub fn room_left_arrow_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    image_assets: Res<MyAssets>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Px(0.0),
                ..room_arrow_button_style()
            },
            ..default()
        })
        .insert(Name::new("Room left arrow button"))
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(CustomButton::RoomArrow.bundle(&asset_server, &image_assets))
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

pub fn room_right_arrow_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    image_assets: Res<MyAssets>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                right: Val::Px(0.0),
                ..room_arrow_button_style()
            },
            ..default()
        })
        .insert(Name::new("Room right arrow button"))
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(CustomButton::RoomArrow.bundle(&asset_server, &image_assets))
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

pub fn room_bottom_arrow_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    image_assets: Res<MyAssets>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                bottom: Val::Px(0.0),
                ..room_arrow_button_style()
            },
            ..default()
        })
        .insert(Name::new("Room bottom arrow button"))
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(CustomButton::RoomArrow.bundle(&asset_server, &image_assets))
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

pub fn room_top_arrow_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    image_assets: Res<MyAssets>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(0.0),
                ..room_arrow_button_style()
            },
            ..default()
        })
        .insert(Name::new("Room top arrow button"))
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(CustomButton::RoomArrow.bundle(&asset_server, &image_assets))
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
