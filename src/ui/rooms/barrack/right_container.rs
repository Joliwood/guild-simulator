use crate::{
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, UniqueId},
    },
    ui::ui_constants::{ARMOR_PATH, SCROLL_PATH, WEAPON_PATH},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

fn spawn_inventory(
    parent: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let inventory_size = player_stats.max_inventory_size;
    let columns = 4;
    let texture_handle_empty_slot: Handle<Image> =
        asset_server.load("images/equipments/empty_inventory_slot.png");

    // Create a parent node for the inventory grid
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|grid| {
            // Create rows for the inventory
            for row_index in 0..(inventory_size + columns - 1) / columns {
                grid.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row_builder| {
                    // Add items from the player's inventory or empty slots
                    for i in 0..columns {
                        let index = row_index * columns + i;

                        if index < inventory_size {
                            if index < player_stats.inventory.len() {
                                let item = &player_stats.inventory[index];
                                let image_atlas_index = match item {
                                    Item::Weapon(weapon) => weapon.image_atlas_index,
                                    Item::Armor(armor) => armor.image_atlas_index,
                                    Item::Scroll(scroll, _) => scroll.image_atlas_index,
                                };

                                let atlas_path = match item {
                                    Item::Weapon(_) => WEAPON_PATH,
                                    Item::Armor(_) => ARMOR_PATH,
                                    Item::Scroll(_, _) => SCROLL_PATH,
                                };

                                let layout = match item {
                                    Item::Weapon(_) | Item::Armor(_) => {
                                        TextureAtlasLayout::from_grid(
                                            UVec2::new(2900, 400),
                                            6,
                                            1,
                                            Some(UVec2::new(0, 0)),
                                            Some(UVec2::new(0, 0)),
                                        )
                                    }
                                    Item::Scroll(_, _) => TextureAtlasLayout::from_grid(
                                        UVec2::new(4320, 1080),
                                        4,
                                        1,
                                        Some(UVec2::new(0, 0)),
                                        Some(UVec2::new(0, 0)),
                                    ),
                                };

                                let tooltip_text = match item {
                                    Item::Weapon(weapon) => {
                                        let mut description = format!("{}\n", weapon.name);

                                        if let Some(endurance) = weapon.endurance {
                                            description
                                                .push_str(&format!("\nEndurance: {}", endurance));
                                        }
                                        if let Some(strength) = weapon.strength {
                                            description
                                                .push_str(&format!("\nStrength: {}", strength));
                                        }
                                        if let Some(intelligence) = weapon.intelligence {
                                            description.push_str(&format!(
                                                "\nIntelligence: {}",
                                                intelligence
                                            ));
                                        }
                                        description.push_str(&format!("\nPrice: {}", weapon.price));

                                        description
                                    }
                                    Item::Armor(armor) => format!("Armor: {}", armor.name),
                                    Item::Scroll(scroll, quantity) => {
                                        let mut description = format!("{}\n", scroll.name);

                                        if let Some(endurance) = scroll.endurance {
                                            description
                                                .push_str(&format!("\nEndurance: {}", endurance));
                                        }
                                        if let Some(strength) = scroll.strength {
                                            description
                                                .push_str(&format!("\nStrength: {}", strength));
                                        }
                                        if let Some(intelligence) = scroll.intelligence {
                                            description.push_str(&format!(
                                                "\nIntelligence: {}",
                                                intelligence
                                            ));
                                        }
                                        description.push_str(&format!("\nQuantity: {}", quantity));
                                        description.push_str(&format!("\nPrice: {}", scroll.price));

                                        description
                                    }
                                };

                                // Spawn button for the item
                                row_builder
                                    .spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(60.),
                                                height: Val::Px(60.),
                                                border: UiRect::all(Val::Px(3.)),
                                                margin: UiRect::all(Val::Px(5.)),
                                                ..default()
                                            },
                                            image: asset_server.load(atlas_path).clone().into(),
                                            border_color: BorderColor(Color::BLACK),
                                            border_radius: BorderRadius::all(Val::Px(10.)),
                                            ..default()
                                        },
                                        TextureAtlas {
                                            index: image_atlas_index.into(),
                                            layout: texture_atlas_layouts.add(layout),
                                        },
                                        Tooltip::cursor(tooltip_text.to_string())
                                            .with_activation(TooltipActivation::IMMEDIATE),
                                    ))
                                    .insert(UniqueId(format!("item_in_inventory")))
                                    .with_children(|button| {
                                        // If the item is a scroll, add a count indicator inside the button
                                        if let Item::Scroll(_, count) = item {
                                            button.spawn(TextBundle {
                                                text: Text::from_section(
                                                    format!("x{}", count),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 14.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                style: Style {
                                                    position_type: PositionType::Absolute,
                                                    bottom: Val::Px(0.0),
                                                    right: Val::Px(5.0),
                                                    ..default()
                                                },
                                                ..default()
                                            });
                                        }
                                    });
                            } else {
                                // Spawn empty inventory slot
                                row_builder
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
                                    .insert(UniqueId(format!("item_in_inventory")));
                            }
                        }
                    }
                });
            }
        });
}

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_test: Handle<Image> = asset_server.load("images/ui/art_v0_buttons.png");
    let layout_test = TextureAtlasLayout::from_grid(
        UVec2::new(4000, 400),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout_test = texture_atlas_layouts.add(layout_test);

    // Container for the inventory
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
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
                                style: Style {
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Percent(100. / 4.),
                                    border: UiRect::all(Val::Px(3.)),
                                    height: Val::Px(40.),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: texture_test.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId(format!("item_in_inventory")))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "All",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Weapons"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100. / 4.),
                                    display: Display::Flex,
                                    border: UiRect::all(Val::Px(3.)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    height: Val::Px(40.),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: texture_test.clone().into(),
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
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Armors"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100. / 4.),
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    border: UiRect::all(Val::Px(3.)),
                                    height: Val::Px(40.),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: texture_test.clone().into(),
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
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // Button for "Scrolls"
                    button_row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100. / 4.),
                                    display: Display::Flex,
                                    border: UiRect::all(Val::Px(3.)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    height: Val::Px(40.),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                image: texture_test.clone().into(),
                                ..default()
                            },
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layout_test.clone(),
                            },
                        ))
                        .insert(UniqueId(format!("item_in_inventory")))
                        .with_children(|b| {
                            b.spawn(TextBundle::from_section(
                                "Scrolls",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });

            // Spawn the inventory below the buttons
            spawn_inventory(parent, player_stats, asset_server, texture_atlas_layouts);
        });
}
