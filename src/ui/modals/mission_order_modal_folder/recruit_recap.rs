use crate::{
    enums::ColorPaletteEnum,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForMission},
    ui::{
        interface::gold_counter::MyAssets,
        rooms::barrack::recruits_list_folder::{
            armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
        },
    },
};
use bevy::prelude::*;

pub fn recruit_recap(
    parent: &mut ChildBuilder,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
    my_assets: &Res<MyAssets>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_stats: &Res<PlayerStats>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Recruits (Image / Name / Stats / Node Container)
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(40.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(10.0),
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
            if selected_recruit_for_mission.0.is_none() {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "No recruit selected",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 16.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                });
                return;
            } else {
                let recruit = selected_recruit_for_mission.clone().0.unwrap();

                // Recruit image
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
                        // Recruit image
                        parent.spawn((
                            ImageBundle {
                                image: my_assets.recruit_picture_atlas.clone().into(),
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Px(400.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                index: recruit.image_atlas_index.into(),
                                layout: recruit_texture_atlas_layout.clone(),
                            },
                        ));
                    });

                parent
                    // Outer container to hold both rows (Name/Level and Stats)
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(5.),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Recruit Name
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Target : {}", recruit.name),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });

                        // Recruit Level
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Level : {}", recruit.level),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });

                        // Recruit Strength
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Str : {}", recruit.strength),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });

                        // Recruit Defense
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Def : {}", recruit.endurance),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });

                        // Recruit Intelligence
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Int : {}", recruit.intelligence),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });

                        // Equipments
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(2.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        top: Val::Px(10.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|equipments_container| {
                                // Top container for weapon and armor
                                equipments_container
                                    .spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            column_gap: Val::Px(2.0),
                                            align_self: AlignSelf::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|top_container| {
                                        weapon_button(
                                            top_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                        );

                                        armor_button(
                                            top_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                        );
                                    });

                                // Bottom container for scrolls
                                equipments_container
                                    .spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(2.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|bottom_container| {
                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            0,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            1,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            2,
                                        );
                                    });
                            });
                    });
            }
        });
}
