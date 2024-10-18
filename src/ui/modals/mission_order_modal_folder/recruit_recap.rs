use crate::{
    enums::ColorPaletteEnum, structs::recruits::SelectedRecruitForMission,
    ui::interface::gold_counter::MyAssets,
};
use bevy::{
    prelude::*,
    ui::{widget::UiImageSize, ContentSize},
};

pub fn recruit_recap(
    parent: &mut ChildBuilder,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
    my_assets: &Res<MyAssets>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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
                ..default()
            },
            background_color: BackgroundColor(ColorPaletteEnum::Primary.as_color()),
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
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                });
                return;
            } else {
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
                                index: selected_recruit_for_mission
                                    .0
                                    .clone()
                                    .unwrap()
                                    .image_atlas_index
                                    .into(),
                                layout: recruit_texture_atlas_layout.clone(),
                            },
                        ));
                    });
                // Recruit name
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        selected_recruit_for_mission.0.clone().unwrap().name,
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });

                // Recruit stats (just an example)
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        format!(
                            "Stats: {}",
                            selected_recruit_for_mission.0.clone().unwrap().level
                        ),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            }
        });
}
