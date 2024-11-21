use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    my_assets::FONT_FIRA,
    structs::{general_structs::UniqueId, recruits::RecruitStats},
};
use bevy::{prelude::*, ui::widget::NodeImageMode};
// use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn map_recruit_card(
    left_container: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    recruit: &RecruitStats,
    recruit_texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    let recruit_attack = recruit.get_total_stats().0;

    left_container
        .spawn((
            Button,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                column_gap: Val::Px(12.),
                width: Val::Percent(100.),
                height: Val::Px(50.),
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
            BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
            BorderRadius::all(Val::Px(10.)),
            ImageNode {
                image: my_assets.load("images/rooms/command_room/recruit_card_3.png"),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .insert((UniqueId("map_recruit_button".to_string()), recruit.clone()))
        // .insert((
        //     UniqueId("assign_recruit_to_mission".to_string()),
        //     recruit.clone(),
        // ))
        .with_children(|parent| {
            // Add an overlay if the recruit is in a mission
            if recruit.state == RecruitStateEnum::InMission
                || recruit.state == RecruitStateEnum::WaitingReportSignature
            {
                parent
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            // column_gap: Val::Px(5.),
                            padding: UiRect {
                                left: Val::Px(42.),
                                ..default()
                            },
                            ..default()
                        },
                        GlobalZIndex(1),
                        BorderRadius::all(Val::Px(10.)),
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                    ))
                    .with_children(|overlay| {
                        overlay.spawn((
                            Text::new(recruit.state.get_description()),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        })
        .with_children(|button| {
            button
                .spawn((
                    Button,
                    Node {
                        // display: Display::Flex,
                        // column_gap: Val::Px(5.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    //                     Tooltip::cursor(
                    //                         "This score represent the
                    // total power of the recruit, based on
                    // his/her stats, equipment and level."
                    //                             .to_string(),
                    //                     )
                    //                     .with_activation(TooltipActivation::IDLE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(recruit_attack.to_string()),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            // Recruit name
            button.spawn((
                Text::new(recruit.name.clone()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            button
                .spawn(Node {
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
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::from_atlas_image(
                            my_assets.load("images/recruits/recruit_picture_atlas.png"),
                            TextureAtlas {
                                index: recruit.image_atlas_index.into(),
                                layout: recruit_texture_atlas_layout.clone(),
                            },
                        ),
                        Node {
                            margin: UiRect {
                                top: Val::Px(-10.),
                                ..default()
                            },
                            width: Val::Percent(100.),
                            height: Val::Px(70. * 2.),
                            ..default()
                        },
                    ));
                });
        });
}
