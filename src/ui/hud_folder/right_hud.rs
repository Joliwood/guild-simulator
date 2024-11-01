use crate::{
    my_assets::MyAssets,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{RecruitCountTrigger, ReputationCountTrigger, ToxicityCountTrigger},
    },
};
use bevy::prelude::*;

pub fn right_hud(
    commands: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: Handle<TextureAtlasLayout>,
) {
    commands
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
                            layout: texture_atlas_layouts.clone(),
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
                            layout: texture_atlas_layouts.clone(),
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
                            layout: texture_atlas_layouts.clone(),
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
                    layout: texture_atlas_layouts.clone(),
                },
            ));
        });
}
