use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    my_assets::MyAssets,
    structs::{general_structs::UniqueId, recruits::RecruitStats},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn map_recruit_card(
    left_container: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    recruit: &RecruitStats,
    recruit_texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    left_container
        .spawn(ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                column_gap: Val::Px(20.),
                width: Val::Percent(100.),
                height: Val::Px(40.),
                padding: UiRect {
                    top: Val::Px(3.),
                    bottom: Val::Px(3.),
                    left: Val::Px(7.),
                    right: Val::Px(3.),
                },
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                ..default()
            },
            // background_color: Color::BLACK.into(),
            image: my_assets.recruit_card.clone().into(),
            border_color: BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
            border_radius: BorderRadius::all(Val::Px(10.)),
            ..default()
        })
        // .insert((
        //     UniqueId("assign_recruit_to_mission".to_string()),
        //     recruit.clone(),
        // ))
        .insert((UniqueId("map_recruit_button".to_string()), recruit.clone()))
        .with_children(|parent| {
            // Add an overlay if the recruit is in a mission
            if recruit.state == RecruitStateEnum::InMission
                || recruit.state == RecruitStateEnum::WaitingReportSignature
            {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            padding: UiRect {
                                left: Val::Px(42.),
                                ..default()
                            },
                            ..default()
                        },
                        z_index: ZIndex::Global(1),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                        ..default()
                    })
                    .with_children(|overlay| {
                        overlay.spawn(TextBundle {
                            text: Text::from_section(
                                recruit.state.get_description(),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        })
        .with_children(|button| {
            button
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Tooltip::cursor(
                        "This score represent the
total power of the recruit, based on
his/her stats, equipment and level."
                            .to_string(),
                    )
                    .with_activation(TooltipActivation::IDLE),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        recruit.get_total_merged_stats().to_string(),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 18.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Recruit name
            button.spawn(TextBundle::from_section(
                recruit.name.clone(),
                TextStyle {
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));

            button
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(72.),
                        position_type: PositionType::Absolute,
                        right: Val::Px(0.),
                        height: Val::Percent(65.),
                        padding: UiRect {
                            left: Val::Px(3.),
                            right: Val::Px(5.),
                            ..default()
                        },
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
                            image: my_assets.recruit_picture_atlas.clone().into(),
                            style: Style {
                                margin: UiRect {
                                    top: Val::Px(-10.),
                                    ..default()
                                },
                                width: Val::Percent(100.),
                                height: Val::Px(70. * 2.),
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
        });
}
