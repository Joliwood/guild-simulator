use crate::{
    my_assets::MyAssets,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{GoldCountTrigger, GuildLvlTrigger, PlayerDayTrigger},
    },
};
use bevy::prelude::*;

pub fn left_hud(
    commands: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
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
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                width: Val::Px(205.0),
                height: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Left Container"))
        .with_children(|left_container| {
            left_container
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
                            index: 3,
                            layout: hud_icons_texture_atlas_layout.clone(),
                        },
                    ));
                    // Adding 3 text elements
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                player_stats.golds.to_string(),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
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
                            style: TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("{}", player_stats.guild_level),
                            style: TextStyle {
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
                            style: TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("{}", player_stats.day),
                            style: TextStyle {
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