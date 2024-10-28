use crate::{my_assets::MyAssets, structs::daily_events_folder::discussions::DailyDiscussion};
use bevy::prelude::*;

pub fn discussion_event_doc(
    commands: &mut Commands,
    my_assets: &Res<MyAssets>,
    discussion: DailyDiscussion,
) {
    // WIP - Must create textures for each discussion
    // let ennemy_layout = TextureAtlasLayout::from_grid(
    //     UVec2::new(1200, 200),
    //     6,
    //     1,
    //     Some(UVec2::new(0, 0)),
    //     Some(UVec2::new(0, 0)),
    // );
    // let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
    //     texture_atlas_layouts.add(ennemy_layout);

    commands
        .spawn(ImageBundle {
            image: my_assets.daily_event_document.clone().into(),
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                width: Val::Px(330.),
                height: Val::Px(500.),
                padding: UiRect {
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(2),
            ..default()
        })
        .with_children(|parent| {
            // Container with flex column layout
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|column| {
                    // Title at the top
                    column.spawn(TextBundle {
                        text: Text::from_section(
                            discussion.title.clone(),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 18.0,
                                color: Color::BLACK,
                            },
                        ),
                        style: Style {
                            margin: UiRect::bottom(Val::Px(8.)),
                            ..default()
                        },
                        ..default()
                    });

                    // Image below the title
                    column.spawn(ImageBundle {
                        image: my_assets.inventory_container.clone().into(),
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Px(150.),
                            margin: UiRect::bottom(Val::Px(8.)),
                            ..default()
                        },
                        z_index: ZIndex::Global(1),
                        ..default()
                    });

                    // Description below the image
                    column.spawn(TextBundle {
                        text: Text::from_section(
                            discussion.description.clone(),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        ),
                        style: Style {
                            margin: UiRect::bottom(Val::Px(12.)),
                            ..default()
                        },
                        ..default()
                    });

                    // Map through answers and display each below the description
                    for answer in discussion.answers.iter() {
                        column.spawn(TextBundle {
                            text: Text::from_section(
                                answer.message.clone(),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::top(Val::Px(4.)),
                                // max_size: Size::new(Val::Percent(90.), Val::Auto),
                                ..default()
                            },
                            ..default()
                        });
                    }
                });
        });
}
