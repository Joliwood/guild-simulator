use crate::{
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, UniqueId},
    },
    utils::{
        get_item_atlas_path, get_item_image_atlas_index, get_item_layout,
        get_item_tooltip_description,
    },
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn spawn_inventory(
    parent: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let inventory_size = player_stats.max_inventory_size;
    let columns = 5;
    let texture_handle_empty_slot: Handle<Image> =
        asset_server.load("images/equipments/empty_inventory_slot.png");

    // Create a parent node for the inventory grid
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Px(350.),
                overflow: Overflow::clip(),
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
                                let item_image_atlas_index = get_item_image_atlas_index(item);
                                let item_atlas_path = get_item_atlas_path(item);
                                let layout = get_item_layout(item);
                                let tooltip_text = get_item_tooltip_description(item);
                                let player_inventory_item_index = index;

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
                                            image: asset_server
                                                .load(item_atlas_path)
                                                .clone()
                                                .into(),
                                            border_color: BorderColor(Color::BLACK),
                                            border_radius: BorderRadius::all(Val::Px(10.)),
                                            ..default()
                                        },
                                        TextureAtlas {
                                            index: item_image_atlas_index.into(),
                                            layout: texture_atlas_layouts.add(layout),
                                        },
                                        Tooltip::cursor(tooltip_text.to_string())
                                            .with_activation(TooltipActivation::IMMEDIATE),
                                    ))
                                    .insert(UniqueId(format!(
                                        "item_in_inventory_{}",
                                        player_inventory_item_index
                                    )))
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
                                    .insert(UniqueId(format!("item_in_inventory_0")));
                            }
                        }
                    }
                });
            }
        });
}
