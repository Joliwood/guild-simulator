use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    structs::{
        general_structs::UniqueId, missions::SelectedMission, player_stats::PlayerStats,
        recruits::RecruitStats,
    },
    ui::{
        interface::gold_counter::MyAssets,
        rooms::barrack::recruits_list_folder::{
            armor_button::armor_button, recruit_endurance::recruit_endurance,
            recruit_intelligence::recruit_intelligence, recruit_strength::recruit_strength,
            scroll_button::scroll_button, weapon_button::weapon_button,
        },
    },
};
use bevy::prelude::*;

pub fn map_recruit_card(
    left_container: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    recruit: &RecruitStats,
    recruit_texture_atlas_layout: Handle<TextureAtlasLayout>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    selected_mission: &mut ResMut<SelectedMission>,
) {
    let victory_percentage: String = match selected_mission.percent_of_victory {
        Some(victory_percentage) => victory_percentage.to_string(),
        None => "".to_string(),
    };

    left_container
        .spawn(ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect {
                    top: Val::Px(15.),
                    bottom: Val::Px(15.),
                    left: Val::Px(3.),
                    right: Val::Px(3.),
                },
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                ..default()
            },
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
                            justify_content: JustifyContent::Center,
                            padding: UiRect {
                                top: Val::ZERO,
                                bottom: Val::ZERO,
                                left: Val::Percent(15.),
                                right: Val::Percent(15.),
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
            // Recruit portrait image (left-most side)
            button.spawn((
                ImageBundle {
                    image: my_assets.recruit_picture_atlas.clone().into(),
                    style: Style {
                        width: Val::Px(30.),
                        height: Val::Px(50.),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: recruit.image_atlas_index.into(),
                    layout: recruit_texture_atlas_layout.clone(),
                },
            ));

            // Container for recruit name and class
            button
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: Val::Px(5.0),
                        width: Val::Px(80.0),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|name_class_container| {
                    // Recruit name
                    name_class_container.spawn(TextBundle::from_section(
                        recruit.name.clone(),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 16.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));

                    // Recruit class
                    name_class_container.spawn(TextBundle::from_section(
                        recruit.class.to_string(),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));
                });

            // Container for recruit stats (strength, armor, intelligence)
            button
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stats_container| {
                    stats_container.spawn(TextBundle::from_section(
                        format!("Str: {}", recruit.get_total_strength()),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));

                    stats_container.spawn(TextBundle::from_section(
                        format!("End: {}", recruit.get_total_endurance()),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));

                    stats_container.spawn(TextBundle::from_section(
                        format!("Int: {}", recruit.get_total_intelligence()),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));
                });

            button
                .spawn(ImageBundle {
                    image: my_assets.set_of_keys_container.clone().into(),
                    style: Style {
                        width: Val::Px(30.),
                        height: Val::Px(30.),
                        display: Display::Flex, // To allow flex positioning
                        align_items: AlignItems::Center, // Align text vertically in the center
                        justify_content: JustifyContent::Center, // Align text horizontally in the center
                        ..default()
                    },
                    ..default()
                })
                .with_children(|frame| {
                    // Image that is 30x30 with centered text inside
                    frame.spawn(TextBundle::from_section(
                        victory_percentage,
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));
                });
        });
}
