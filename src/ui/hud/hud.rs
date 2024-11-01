use super::left_hud::left_hud;
use crate::{
    enums::ColorPaletteEnum,
    my_assets::MyAssets,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{RecruitCountTrigger, ReputationCountTrigger, ToxicityCountTrigger},
    },
};
use bevy::prelude::*;

pub fn hud(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let hud_icons_layout = TextureAtlasLayout::from_grid(
        UVec2::new(4000, 500),
        8,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let hud_icons_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(hud_icons_layout);

    commands
        // Main Container
        .spawn(ImageBundle {
            image: my_assets.hud.clone().into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                display: Display::Flex,
                ..default()
            },
            z_index: ZIndex::Global(3),
            ..default()
        })
        .insert(Name::new("HUD"))
        // Left Container
        .with_children(|parent| {
            left_hud(parent, &my_assets, &player_stats, texture_atlas_layouts);

            // Middle Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        height: Val::Px(35.0),
                        width: Val::Px(165.0),
                        margin: UiRect {
                            right: Val::Px(2.),
                            ..default()
                        },
                        column_gap: Val::Px(7.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Middle Container"))
                .with_children(|middle_container| {
                    // Adding 3 button elements
                    for i in 1..=3 {
                        middle_container
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Percent(100.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(
                                    ColorPaletteEnum::Brown.as_color(),
                                ),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                        i.to_string(),
                                        TextStyle {
                                            font: my_assets.fira_sans_bold.clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
            // Right Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        width: Val::Px(208.0),
                        height: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Right Container"))
                .with_children(|right_container| {
                    // Recruits length
                    right_container
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                column_gap: Val::Px(5.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: my_assets.hud_icon_atlas.clone().into(),
                                    style: Style {
                                        width: Val::Px(16.),
                                        height: Val::Px(16.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TextureAtlas {
                                    index: 0,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ));
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        player_stats.recruits.len().to_string(),
                                        TextStyle {
                                            font: my_assets.fira_sans_bold.clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(RecruitCountTrigger);
                        });

                    // Reputation score
                    right_container
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                column_gap: Val::Px(5.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: my_assets.hud_icon_atlas.clone().into(),
                                    style: Style {
                                        width: Val::Px(16.),
                                        height: Val::Px(16.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TextureAtlas {
                                    index: 6,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ));
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        player_stats.reputation.to_string(),
                                        TextStyle {
                                            font: my_assets.fira_sans_bold.clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(ReputationCountTrigger);
                        });

                    // Toxicity score
                    right_container
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                column_gap: Val::Px(5.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: my_assets.hud_icon_atlas.clone().into(),
                                    style: Style {
                                        width: Val::Px(16.),
                                        height: Val::Px(16.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TextureAtlas {
                                    index: 5,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ));
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        player_stats.toxicity.to_string(),
                                        TextStyle {
                                            font: my_assets.fira_sans_bold.clone(),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(ToxicityCountTrigger);
                        });

                    right_container.spawn((
                        ButtonBundle {
                            image: my_assets.hud_icon_atlas.clone().into(),
                            style: Style {
                                width: Val::Px(16.),
                                height: Val::Px(16.),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: 7,
                            layout: hud_icons_texture_atlas_layout.clone(),
                        },
                    ));
                });
        });
}
