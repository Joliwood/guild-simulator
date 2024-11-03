use crate::{
    my_assets::MyAssets,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{GoldCountTrigger, GuildLvlTrigger, PlayerDayTrigger},
    },
};
use bevy::prelude::*;
use pyri_tooltip::Tooltip;

pub fn left_hud(
    commands: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &Handle<TextureAtlasLayout>,
) {
    commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            width: Val::Px(205.0),
            height: Val::Px(25.0),
            ..default()
        })
        .insert(Name::new("Left Container"))
        .with_children(|left_container| {
            left_container
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            column_gap: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    },
                    Tooltip::cursor("Your golds"),
                ))
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
                            index: 3,
                            layout: texture_atlas_layouts.clone(),
                        },
                    ));
                    // Adding 3 text elements
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                player_stats.golds.to_string(),
                                TextFont {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    // color: Color::WHITE,
                                    text_color,
                                },
                                col,
                            ),
                            ..default()
                        })
                        .insert(GoldCountTrigger);
                });

            left_container
                .spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "Lvl : ".to_string(),
                            style: TextFont {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                // color: Color::WHITE,
                                text_color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("{}", player_stats.guild_level),
                            style: TextFont {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                        },
                    ]),
                    ..default()
                })
                .insert(GuildLvlTrigger);

            left_container
                .spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "Day : ".to_string(),
                            style: TextFont {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("{}", player_stats.day),
                            style: TextFont {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                        },
                    ]),
                    ..default()
                })
                .insert(PlayerDayTrigger);
        });
}
