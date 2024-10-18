use crate::{
    enums::ColorPaletteEnum, structs::missions::Mission, ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

pub fn mission_recap(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    mission: &Mission,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let ennemy_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1200, 200),
        6,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(ennemy_layout);

    // Mission Image and Description (A / B)
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                justify_content: JustifyContent::FlexStart,
                width: Val::Percent(50.0),
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect {
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    bottom: Val::Px(10.),
                },
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::all(Val::Px(10.)),
            ..default()
        })
        .with_children(|parent| {
            // Mission image
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: my_assets.ennemy_image_handle.clone().into(),
                            style: Style {
                                margin: UiRect {
                                    top: Val::Px(-100.0),
                                    ..default()
                                },
                                width: Val::Percent(100.0),
                                height: Val::Px(450.),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: mission.ennemy.image_atlas_index.into(),
                            layout: ennemy_texture_atlas_layout.clone(),
                        },
                    ));
                });

            // Mission description
            parent.spawn(TextBundle {
                text: Text::from_section(
                    mission.description.clone(),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            parent
                // Outer container to hold both rows (Name/Level and Stats)
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column, // Organize rows in a column
                        justify_content: JustifyContent::FlexStart,
                        row_gap: Val::Px(5.),
                        width: Val::Percent(100.0), // Full width
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Row 1: Name (left) and Level (right)
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Target : {}", mission.ennemy.name),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // Enemy Level
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Level : {}", mission.ennemy.level),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // Enemy Strength
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Str : {}", mission.ennemy.strength),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // Enemy Defense
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Def : {}", mission.ennemy.endurance),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // Enemy Intelligence
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Int : {}", mission.ennemy.intelligence),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
        });
}
