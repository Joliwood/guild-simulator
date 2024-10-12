use super::spawn_inventory::spawn_inventory;
use crate::{
    enums::ColorPaletteEnum,
    structs::general_structs::{PlayerStats, UniqueId},
    ui::{
        interface::gold_counter::MyAssets, styles::buttons_styles::inventory_filter_button_style,
    },
};
use bevy::prelude::*;

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // let texture_test: Handle<Image> = my_assets.load("images/ui/art_v0_buttons.png");
    let layout_test = TextureAtlasLayout::from_grid(
        UVec2::new(4000, 400),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout_test = texture_atlas_layouts.add(layout_test);

    // let texture_handle_inventory_container: Handle<Image> =
    //     my_assets.load("images/rooms/barrack/inventory_container.png");

    // Container for the inventory
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(400.),
                height: Val::Px(450.),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Room barrack > inventory"))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Inventory",
                TextStyle {
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 30.,
                    color: ColorPaletteEnum::DarkBrown.as_color(),
                },
            ));

            // Create a row for the filter buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|button_row| {
                    // Button for "All"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: inventory_filter_button_style(),
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: my_assets.art_v0_buttons.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId("item_in_inventory".to_string()))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "All",
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Weapons"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: inventory_filter_button_style(),
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: my_assets.art_v0_buttons.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId("item_in_inventory".to_string()))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "Weapons",
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Armors"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: inventory_filter_button_style(),
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: my_assets.art_v0_buttons.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId("item_in_inventory".to_string()))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "Armors",
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Scrolls"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: inventory_filter_button_style(),
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: my_assets.art_v0_buttons.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId("item_in_inventory".to_string()))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "Scrolls",
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });

            // Spawn the inventory below the buttons
            spawn_inventory(parent, player_stats, my_assets, texture_atlas_layouts);
        });
}
