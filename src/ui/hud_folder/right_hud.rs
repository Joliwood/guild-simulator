use crate::{
    my_assets::FONT_FIRA,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{RecruitCountTrigger, ReputationCountTrigger, ToxicityCountTrigger},
    },
};
use bevy::prelude::*;
use pyri_tooltip::Tooltip;

pub fn right_hud(
    commands: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: Handle<TextureAtlasLayout>,
) {
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            width: Val::Px(208.0),
            height: Val::Px(25.0),
            ..default()
        })
        .insert(Name::new("Right Container"))
        .with_children(|right_container| {
            // Recruits length
            right_container
                .spawn((
                    Button,
                    Node {
                        display: Display::Flex,
                        column_gap: Val::Px(5.),
                        ..default()
                    },
                    // Tooltip::cursor("You have this many recruits"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        UiImage::from_atlas_image(
                            my_assets.load("images/hud/hud_icon_atlas.png"),
                            TextureAtlas {
                                index: 0,
                                layout: texture_atlas_layouts.clone(),
                            },
                        ),
                        Node {
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            Text::new(player_stats.recruits.len().to_string()),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ))
                        .insert(RecruitCountTrigger);
                });

            // Reputation score
            right_container
                .spawn((
                    Button,
                    Node {
                        display: Display::Flex,
                        column_gap: Val::Px(5.),
                        ..default()
                    },
                    // Tooltip::cursor("Your reputation score"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        UiImage::from_atlas_image(
                            my_assets.load("images/hud/hud_icon_atlas.png"),
                            TextureAtlas {
                                index: 6,
                                layout: texture_atlas_layouts.clone(),
                            },
                        ),
                        Node {
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            Text::new(player_stats.reputation.to_string()),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ))
                        .insert(ReputationCountTrigger);
                });

            // Toxicity score
            right_container
                .spawn((
                    Button,
                    Node {
                        display: Display::Flex,
                        column_gap: Val::Px(5.),
                        ..default()
                    },
                    // Tooltip::cursor("Your toxicity score"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        UiImage::from_atlas_image(
                            my_assets.load("images/hud/hud_icon_atlas.png"),
                            TextureAtlas {
                                index: 5,
                                layout: texture_atlas_layouts.clone(),
                            },
                        ),
                        Node {
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            Text::new(player_stats.toxicity.to_string()),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ))
                        .insert(ToxicityCountTrigger);
                });

            right_container.spawn((
                Button,
                Node {
                    width: Val::Px(16.),
                    height: Val::Px(16.),
                    ..default()
                },
                UiImage::from_atlas_image(
                    my_assets.load("images/hud/hud_icon_atlas.png"),
                    TextureAtlas {
                        index: 7,
                        layout: texture_atlas_layouts.clone(),
                    },
                ),
                // Tooltip::cursor("Settings menu (not implemented yet)"),
            ));
        });
}
