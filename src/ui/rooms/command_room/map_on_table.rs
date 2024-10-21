use crate::{
    custom_components::CustomButton,
    structs::{general_structs::UniqueId, maps::Map, missions::Missions},
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

// External function for the center area (1 big child)
pub fn map_on_table(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    map: &Option<Map>,
    missions: &Res<Missions>,
) {
    parent
        .spawn(NodeBundle {
            // image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            ..default()
        })
        .with_children(|button| {
            if map.is_some() {
                let missions = missions.get_missions_by_ids(map.clone().unwrap().map_mission_ids);
                button
                    .spawn(ImageBundle {
                        image: my_assets.get_image_map(map.clone().unwrap().image).into(),
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|map| {
                        // Generate buttons for each mission
                        for mission in missions.iter().filter(|mission| mission.unlocked) {
                            if mission.recruit_send.is_none() {
                                map.spawn(
                                    CustomButton::MissionOnMap
                                        .mission_bundle(my_assets, mission.id),
                                )
                                .insert(UniqueId("select_mission_button".to_string()))
                                .insert(mission.clone());
                            } else {
                                map.spawn(
                                    CustomButton::MissionOnMap
                                        .mission_bundle(my_assets, mission.id),
                                )
                                .with_children(|button| {
                                    // Black filter overlay with centered text
                                    button
                                        .spawn(NodeBundle {
                                            style: Style {
                                                position_type: PositionType::Absolute,
                                                top: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                                left: Val::Px(0.0),
                                                right: Val::Px(0.0),
                                                padding: UiRect::all(Val::Px(10.)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8)
                                                .into(),
                                            ..default()
                                        })
                                        .with_children(|overlay| {
                                            overlay.spawn(TextBundle {
                                                text: Text::from_section(
                                                    "A recruit has already been sent",
                                                    TextStyle {
                                                        font: my_assets.fira_sans_bold.clone(),
                                                        font_size: 18.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });
                                });
                            }
                        }
                    });
            }
        });
}
