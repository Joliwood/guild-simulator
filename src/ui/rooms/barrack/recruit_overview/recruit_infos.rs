use crate::{
    enums::ColorPaletteEnum,
    structs::{
        general_structs::{SelectedRecruit, UniqueId},
        trigger_structs::SelectedRecruitTrigger,
    },
};
use bevy::prelude::*;

pub fn recruit_infos(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_image_handle: Handle<Image> =
        asset_server.load("images/rooms/barrack/recruit_infos.png");
    let texture_handle_empty_slot: Handle<Image> =
        asset_server.load("images/equipments/empty_inventory_slot.png");

    parent
        .spawn(ImageBundle {
            image: frame_image_handle.into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // width: Val::Percent(100.0),
                width: Val::Auto,
                height: Val::Auto,
                padding: UiRect {
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                },
                ..default()
            },
            z_index: ZIndex::Global(2),
            ..default()
        })
        .insert(Name::new("Barrack > recruit overview > recruit infos"))
        .with_children(|parent| {
            // Top container (holds Weapons and Armor buttons)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|top_container| {
                    // Left column (Weapon)
                    top_container
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|weapon_column| {
                            // Header for Weapon
                            weapon_column.spawn(TextBundle::from_section(
                                "Weapon",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            // Weapon button
                            weapon_column
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.),
                                        height: Val::Px(60.),
                                        border: UiRect::all(Val::Px(3.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    image: texture_handle_empty_slot.clone().into(),
                                    ..default()
                                })
                                .insert(UniqueId(format!("item_in_inventory_")));
                        });

                    // Right column (Armor)
                    top_container
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|armor_column| {
                            // Header for Armor
                            armor_column.spawn(TextBundle::from_section(
                                "Armor",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            // Armor button
                            armor_column
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.),
                                        height: Val::Px(60.),
                                        border: UiRect::all(Val::Px(3.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    image: texture_handle_empty_slot.clone().into(),
                                    ..default()
                                })
                                .insert(UniqueId(format!("item_in_inventory_")));
                        });
                });

            // Bottom container (holds Scrolls header and three buttons)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        // margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom_container| {
                    // Header for Scrolls
                    bottom_container.spawn(TextBundle::from_section(
                        "Scrolls",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));

                    // Row container for buttons
                    bottom_container
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|button_row| {
                            // First button
                            button_row
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.),
                                        height: Val::Px(60.),
                                        border: UiRect::all(Val::Px(3.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    image: texture_handle_empty_slot.clone().into(),
                                    ..default()
                                })
                                .insert(UniqueId(format!("item_in_inventory_")));

                            // Second button
                            button_row
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.),
                                        height: Val::Px(60.),
                                        border: UiRect::all(Val::Px(3.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    image: texture_handle_empty_slot.clone().into(),
                                    ..default()
                                })
                                .insert(UniqueId(format!("item_in_inventory_")));

                            // Third button
                            button_row
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.),
                                        height: Val::Px(60.),
                                        border: UiRect::all(Val::Px(3.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    image: texture_handle_empty_slot.clone().into(),
                                    ..default()
                                })
                                .insert(UniqueId(format!("item_in_inventory_")));
                        });
                });
        });
}
