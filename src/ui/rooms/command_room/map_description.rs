use crate::{my_assets::MyAssets, structs::maps::Map};
use bevy::prelude::*;

pub fn map_description(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_map: &Option<Map>,
) {
    parent
        .spawn(ImageBundle {
            image: my_assets.map_description.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(10.),
                align_items: AlignItems::FlexStart,
                width: Val::Percent(100.),
                height: Val::Percent(35.),
                padding: UiRect {
                    left: Val::Px(25.),
                    right: Val::Px(25.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|column| {
            if let Some(map) = selected_map {
                column.spawn(TextBundle {
                    text: Text::from_section(
                        map.name.clone(),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 16.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });

                column.spawn(TextBundle {
                    text: Text::from_section(
                        map.description.clone(),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            }
        });
}
