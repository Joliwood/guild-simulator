use crate::{
    enums::{ColorPaletteEnum, TextureAtlasLayoutEnum},
    my_assets::{get_item_atlas_path, FONT_FIRA},
    structs::{
        equipments::ItemEnum, player_stats::PlayerStats, recruits::SelectedRecruitForEquipment,
        trigger_structs::ItemInInventoryTrigger,
    },
    utils::{get_item_image_atlas_index, get_layout},
};
use bevy::{prelude::*, ui::widget::NodeImageMode};
use std::cmp::Ordering;
// use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn spawn_inventory(
    parent: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
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
                                let item_layout = get_layout(TextureAtlasLayoutEnum::Item(item));
                                // let tooltip_text = get_item_tooltip_description(item);
                                let item_atlas_path = get_item_atlas_path(item);

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
                                        ImageNode::from_atlas_image(
                                            my_assets.load(item_atlas_path),
                                            TextureAtlas {
                                                index: item_image_atlas_index.into(),
                                                layout: texture_atlas_layouts.add(item_layout),
                                            },
                                        )
                                        .with_mode(NodeImageMode::Stretch),
                                        ItemInInventoryTrigger(Some(item.clone())),
                                    ))
                                    .with_children(|button| {
                                        // If the item is a scroll, add a count indicator inside the button
                                        if let ItemEnum::Scroll(_, count) = item {
                                            button.spawn((
                                                Text::new(format!("x{}", count)),
                                                TextFont {
                                                    font: my_assets.load(FONT_FIRA),
                                                    font_size: 12.0,
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

                                        if let Some(selected_recruit) =
                                            selected_recruit_for_equipment.0.as_ref()
                                        {
                                            let comparator = match item {
                                                ItemEnum::Weapon(weapon) => {
                                                    if let Some(selected_recruit_weapon) =
                                                        &selected_recruit.recruit_inventory.weapon
                                                    {
                                                        match weapon
                                                            .power
                                                            .cmp(&selected_recruit_weapon.power)
                                                        {
                                                            Ordering::Greater => "+",
                                                            Ordering::Less => "-",
                                                            Ordering::Equal => "=",
                                                        }
                                                    } else {
                                                        "+"
                                                    }
                                                }
                                                ItemEnum::Armor(armor) => {
                                                    if let Some(selected_recruit_armor) =
                                                        &selected_recruit.recruit_inventory.armor
                                                    {
                                                        match armor
                                                            .power
                                                            .cmp(&selected_recruit_armor.power)
                                                        {
                                                            Ordering::Greater => "+",
                                                            Ordering::Less => "-",
                                                            Ordering::Equal => "=",
                                                        }
                                                    } else {
                                                        "+"
                                                    }
                                                }
                                                // We do not show the comparator for scrolls
                                                _ => "",
                                            };

                                            // Spawning the button with the computed comparator
                                            button.spawn((
                                                Text::new(comparator),
                                                TextFont {
                                                    font: my_assets.load(FONT_FIRA),
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                                TextColor(ColorPaletteEnum::SunnyLight.as_color()),
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    top: Val::Px(0.0),
                                                    right: Val::Px(5.0),
                                                    ..default()
                                                },
                                            ));
                                        }
                                    });
                            } else {
                                // Spawn empty inventory slot
                                row_builder.spawn((
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
                                    ImageNode {
                                        image: my_assets
                                            .load("images/equipments/empty_inventory_slot.png"),
                                        ..Default::default()
                                    },
                                ));
                            }
                        }
                    }
                });
            }
        });
}
