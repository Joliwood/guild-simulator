use crate::{
    custom_components::CustomButton,
    enums::ColorPaletteEnum,
    structs::{
        general_structs::UniqueId, maps::Maps, missions::Missions,
        trigger_structs::ResetRoomTrigger,
    },
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::node_container_style},
};
use bevy::prelude::*;

use super::{map_list::map_list, map_on_table::map_on_table};

pub fn room_command_room(
    my_assets: &Res<MyAssets>,
    commands: &mut Commands,
    missions: Res<Missions>,
    maps: Res<Maps>,
) {
    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(Name::new("Command room"))
        .insert(ResetRoomTrigger)
        // Background Image for the Command Room
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: my_assets.command_room_background.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                z_index: ZIndex::Global(-1),
                ..default()
            });
        })
        // Command Table with all child elements inside it
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ImageBundle {
                    image: my_assets.command_table.clone().into(),
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(1170.0),
                        height: Val::Px(650.0),
                        padding: UiRect::all(Val::Px(80.)),
                        ..default()
                    },
                    z_index: ZIndex::Global(0),
                    ..default()
                })
                .with_children(|table| {
                    // Left Column
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexStart,
                                width: Val::Percent(20.),
                                height: Val::Percent(100.),
                                overflow: Overflow {
                                    x: OverflowAxis::Hidden,
                                    y: OverflowAxis::Hidden,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|left_column| {
                            map_list(left_column, &my_assets, &maps);

                            // Map description
                            map_list(left_column, &my_assets, &maps);

                            // Map objectives ?? Not sure for this feature
                        });

                    // Center Area (Big node)
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|center| {
                            // External function for the center area
                            map_on_table(center, my_assets);
                        });

                    // Right Column
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexEnd,
                                width: Val::Percent(20.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|left_column| {
                            // Description of the selected mission + loots
                            map_list(left_column, &my_assets, &maps);

                            // Change of victory + button to start it
                            map_list(left_column, &my_assets, &maps);

                            // Selected recruit
                        });
                });
        });
}
