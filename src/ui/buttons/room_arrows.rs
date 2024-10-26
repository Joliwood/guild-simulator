use crate::{
    custom_components::CustomButton, my_assets::MyAssets, structs::general_structs::UniqueId,
    ui::styles::containers_styles::room_arrow_button_style,
};
use bevy::prelude::*;

pub fn room_left_arrow_button(my_assets: Res<MyAssets>, mut commands: Commands) {
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
                .spawn(CustomButton::RoomArrow.bundle(&my_assets))
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_left_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "<",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_right_arrow_button(my_assets: Res<MyAssets>, mut commands: Commands) {
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
                .spawn(CustomButton::RoomArrow.bundle(&my_assets))
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_right_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        ">",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_bottom_arrow_button(my_assets: Res<MyAssets>, mut commands: Commands) {
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
                .spawn(CustomButton::RoomArrow.bundle(&my_assets))
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_bottom_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "V",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn room_top_arrow_button(my_assets: Res<MyAssets>, mut commands: Commands) {
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
                .spawn(CustomButton::RoomArrow.bundle(&my_assets))
                // Unique ID which will serve the hover / click button
                .insert(UniqueId("room_top_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "^",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
