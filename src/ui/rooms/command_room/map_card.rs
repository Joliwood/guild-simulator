use crate::{enums::ColorPaletteEnum, my_assets::MyAssets, structs::maps::Map};
use bevy::prelude::*;

pub fn map_card(
    column: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    map: &Map,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let map_layout = TextureAtlasLayout::from_grid(
        UVec2::new(270, 400),
        1,
        2,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let map_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(map_layout);
    let icon_atlas_index = map.map_type.get_icon_atlas_index();

    let map_type_layout = TextureAtlasLayout::from_grid(
        UVec2::new(401, 101),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let map_type_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(map_type_layout);

    let missions_finished_number = map.get_finished_missions_number();

    column
        .spawn((
            Button,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Px(70.0),
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow {
                    x: OverflowAxis::Clip,
                    y: OverflowAxis::Clip,
                },
                ..default()
            },
            ZIndex(2),
            BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
            BorderRadius::all(Val::Px(5.)),
            UiImage {
                image: my_assets.map_card.clone().into(),
                ..default()
            },
        ))
        .with_children(|map_container| {
            // Map Image
            map_container.spawn((
                UiImage::from_atlas_image(
                    my_assets.map_atlas.clone().into(),
                    TextureAtlas {
                        index: map.image_atlas_index.into(),
                        layout: map_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(3.),
                    bottom: Val::Px(10.),
                    left: Val::Px(3.),
                    height: Val::Px(70. - 6.),
                    aspect_ratio: Some(270. / 200.),
                    ..default()
                },
                ZIndex(1),
            ));

            // Map Name (Top-Right)
            map_container.spawn((
                Text::new(map.name.clone()),
                TextFont {
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::FlexEnd,
                    top: Val::Px(3.),
                    right: Val::Px(7.),
                    ..default()
                },
            ));

            // Bottom-Right Container with 2 Icons
            map_container
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.),
                    right: Val::Px(10.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    row_gap: Val::Px(8.),
                    ..default()
                })
                .with_children(|icon_container| {
                    icon_container.spawn((
                        Text::new(map.recommanded_power_level.to_string()),
                        TextFont {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));

                    if map.limited_in_time {
                        icon_container.spawn((
                            UiImage {
                                image: my_assets.limited_time.clone().into(),
                                ..default()
                            },
                            Node {
                                width: Val::Px(18.),
                                height: Val::Px(18.),
                                ..default()
                            },
                        ));
                    } else {
                        let missions_finished_text =
                            format!("{}/{}", missions_finished_number, map.map_mission_ids.len());
                        icon_container.spawn((
                            Text::new(missions_finished_text),
                            TextFont {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));
                    };
                });

            // Center text
            map_container.spawn((
                UiImage::from_atlas_image(
                    my_assets.map_type_atlas.clone().into(),
                    TextureAtlas {
                        index: icon_atlas_index.into(),
                        layout: map_type_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(88.),
                    bottom: Val::Px(6.),
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    ..default()
                },
            ));
        });
}
