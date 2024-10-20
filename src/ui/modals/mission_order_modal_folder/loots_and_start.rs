use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

use crate::{
    custom_components::CustomButton,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
    },
    ui::interface::gold_counter::MyAssets,
};

pub fn loots_and_start(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    missions: &Res<Missions>,
    selected_mission: &Res<SelectedMission>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let selected_mission_id = selected_mission.mission.as_ref().unwrap().id;
    let mission_loots = missions
        .get_mission_by_id(selected_mission_id)
        .unwrap()
        .loots;

    // Loots (Text / Loot Icons)
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(20.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Loots in text
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Loots :",
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
                            for loot in mission_loots.0.iter() {
                                let item_image_atlas_index = loot.get_atlas_index();
                                let layout = loot.get_item_layout();
                                let tooltip_text = loot.get_item_loot_tooltip_description();
                                parent.spawn((
                                    ButtonBundle {
                                        image: my_assets
                                            .get_item_loot_atlas_path(&loot.item)
                                            .clone()
                                            .into(),
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

            // Button inside the middle container
            parent
                .spawn(CustomButton::MissionStart.bundle(my_assets))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Start the mission",
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        ),
                        style: Style {
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        },
                        ..default()
                    });
                })
                .insert(if selected_mission.recruit_id.is_some() {
                    UniqueId("start_mission".to_string())
                } else {
                    UniqueId("".to_string())
                });
        });
}
