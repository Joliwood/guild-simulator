use crate::{my_assets::MyAssets, structs::maps::Map};
use bevy::prelude::*;

pub fn map_card(column: &mut ChildBuilder, my_assets: &Res<MyAssets>, map: &Map) {
    column
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Px(40.0),
                // margin: UiRect::all(Val::Px(5.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|map_container| {
            // Image bundle for the map
            map_container.spawn(ImageBundle {
                image: my_assets.recap_guild_scroll.clone().into(), // Replace with actual map image
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            });

            // Text bundle centered on top of the image
            map_container.spawn(TextBundle {
                text: Text::from_section(
                    map.name.clone(), // Assuming `map.name` contains the name of the map
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
