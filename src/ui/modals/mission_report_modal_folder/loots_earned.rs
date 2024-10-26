use crate::{my_assets::MyAssets, structs::missions::MissionReport};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn loots_earned(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    golds_gained: i32,
    experience_gained: u32,
    mission_report: &MissionReport,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Loots Text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Loots",
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 18.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Loots in text
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!(
                                "+ {} Golds | + {} XP for recruit",
                                golds_gained, experience_gained
                            ),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // Loots in display row
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for loot in mission_report.loots.iter() {
                                let item_image_atlas_index = loot.get_atlas_index();
                                let layout = loot.get_item_layout();
                                let tooltip_text = loot.get_item_loot_tooltip_description();
                                parent.spawn((
                                    ButtonBundle {
                                        image: my_assets.get_item_atlas_path(loot).clone().into(),
                                        style: Style {
                                            width: Val::Px(50.0),
                                            height: Val::Px(50.0),
                                            border: UiRect::all(Val::Px(3.)),
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        border_radius: BorderRadius::all(Val::Px(10.)),
                                        ..default()
                                    },
                                    TextureAtlas {
                                        index: item_image_atlas_index.into(),
                                        layout: texture_atlas_layouts.add(layout),
                                    },
                                    Tooltip::cursor(tooltip_text.to_string())
                                        .with_activation(TooltipActivation::IMMEDIATE),
                                ));
                            }
                        });
                });
        });
}
