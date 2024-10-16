use crate::{structs::maps::Map, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

pub fn map_description(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_map: &Option<Map>,
) {
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                padding: UiRect {
                    left: Val::Px(30.),
                    right: Val::Px(30.),
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
                            font_size: 16.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            }
        });
}
