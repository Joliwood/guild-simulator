use crate::{
    custom_components::CustomButton,
    enums::MapImageEnum,
    my_assets::FONT_FIRA,
    structs::{general_structs::UniqueId, maps::Map, missions::Missions},
};
use bevy::prelude::*;

// External function for the center area (1 big child)
pub fn map_on_table(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    map: &Option<Map>,
    missions: &Res<Missions>,
) {
    parent
        .spawn(Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::all(Val::Px(20.)),
            ..default()
        })
        .with_children(|button| {
            if let Some(map) = map {
                let missions = missions.get_missions_by_ids(&map.map_mission_ids);
                button
                    .spawn((
                        UiImage {
                            image: my_assets.load(MapImageEnum::get_path(&map.image)),
                            ..default()
                        },
                        Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                    ))
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
                                        .spawn((
                                            Node {
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
                                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                                        ))
                                        .with_children(|overlay| {
                                            overlay.spawn((
                                                Text::new("A recruit has already been sent"),
                                                TextFont {
                                                    font: my_assets.load(FONT_FIRA),
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ));
                                        });
                                });
                            }
                        }
                    });
            }
        });
}
