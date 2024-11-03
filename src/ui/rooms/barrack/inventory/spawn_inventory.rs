use crate::{
    my_assets::MyAssets,
    structs::{equipments::ItemEnum, general_structs::UniqueId, player_stats::PlayerStats},
    utils::{get_item_image_atlas_index, get_item_layout, get_item_tooltip_description},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn spawn_inventory(
    parent: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    my_assets: &Res<MyAssets>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let inventory_size = player_stats.max_inventory_size;
    let columns = 5;

    // Create a parent node for the inventory grid
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Px(350.),
            overflow: Overflow::clip(),
            ..default()
        })
        .with_children(|grid| {
            // Create rows for the inventory
            for row_index in 0..inventory_size.div_ceil(columns) {
                grid.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
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
                                let layout = get_item_layout(item);
                                let tooltip_text = get_item_tooltip_description(item);

                                // Spawn button for the item
                                row_builder
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(60.),
                                            height: Val::Px(60.),
                                            border: UiRect::all(Val::Px(3.)),
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        BorderColor(Color::BLACK),
                                        BorderRadius::all(Val::Px(10.)),
                                        UiImage::from_atlas_image(
                                            my_assets.get_item_atlas_path(item).clone().into(),
                                            TextureAtlas {
                                                index: item_image_atlas_index.into(),
                                                layout: texture_atlas_layouts.add(layout),
                                            },
                                        ),
                                        // Tooltip::cursor(tooltip_text.to_string())
                                        //     .with_activation(TooltipActivation::IMMEDIATE),
                                    ))
                                    .insert(UniqueId("item_in_inventory".to_string()))
                                    .insert(item.clone())
                                    .with_children(|button| {
                                        // If the item is a scroll, add a count indicator inside the button
                                        if let ItemEnum::Scroll(_, count) = item {
                                            button.spawn((
                                                Text::new(format!("x{}", count)),
                                                TextFont {
                                                    font: my_assets.fira_sans_bold.clone(),
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    bottom: Val::Px(0.0),
                                                    right: Val::Px(5.0),
                                                    ..default()
                                                },
                                            ));
                                        }
                                    });
                            } else {
                                // Spawn empty inventory slot
                                row_builder
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(60.),
                                            height: Val::Px(60.),
                                            border: UiRect::all(Val::Px(3.)),
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        BorderColor(Color::BLACK),
                                        BorderRadius::all(Val::Px(10.)),
                                        UiImage {
                                            image: my_assets.empty_inventory_slot.clone(),
                                            ..default()
                                        },
                                    ))
                                    .insert(UniqueId("item_in_inventory0".to_string()));
                            }
                        }
                    }
                });
            }
        });
}
