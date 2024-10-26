use crate::{my_assets::MyAssets, structs::maps::Maps};
use bevy::prelude::*;

use super::map_card::map_card;

pub fn map_list(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    maps: &Res<Maps>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create a vertical container for the map list
    parent
        .spawn(NodeBundle {
            // image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(65.),
                padding: UiRect {
                    left: Val::Px(1.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|column| {
            // Loop through each map and create an ImageBundle with text overlay for each
            for map in maps.0.iter().filter(|map| map.unlocked) {
                map_card(column, my_assets, map, texture_atlas_layouts);
                // column
                //     .spawn(NodeBundle {
                //         style: Style {
                //             flex_direction: FlexDirection::Column,
                //             justify_content: JustifyContent::Center,
                //             align_items: AlignItems::Center,
                //             width: Val::Percent(100.),
                //             height: Val::Px(40.0),
                //             // margin: UiRect::all(Val::Px(5.0)),
                //             ..Default::default()
                //         },
                //         ..Default::default()
                //     })
                //     .with_children(|map_container| {
                //         // Image bundle for the map
                //         map_container.spawn(ImageBundle {
                //             image: my_assets.recap_guild_scroll.clone().into(), // Replace with actual map image
                //             style: Style {
                //                 width: Val::Percent(100.),
                //                 height: Val::Percent(100.),
                //                 ..default()
                //             },
                //             ..default()
                //         });

                //         // Text bundle centered on top of the image
                //         map_container.spawn(TextBundle {
                //             text: Text::from_section(
                //                 map.name.clone(), // Assuming `map.name` contains the name of the map
                //                 TextStyle {
                //                     font: my_assets.fira_sans_bold.clone(),
                //                     font_size: 16.0,
                //                     color: Color::WHITE,
                //                 },
                //             ),
                //             style: Style {
                //                 position_type: PositionType::Absolute,
                //                 align_self: AlignSelf::Center,
                //                 ..Default::default()
                //             },
                //             ..Default::default()
                //         });
                //     });
            }
        });
}
