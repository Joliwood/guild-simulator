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
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
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
                                map.spawn(CustomButton::Primary.bundle(my_assets))
                                    .insert(UniqueId("select_mission_button".to_string()))
                                    .insert(mission.clone())
                                    .with_children(|button| {
                                        button.spawn(TextBundle {
                                            text: Text::from_section(
                                                format!(
                                                    "Mission {}: Level {}",
                                                    mission.name, mission.level
                                                ),
                                                TextStyle {
                                                    font: my_assets.fira_sans_bold.clone(),
                                                    font_size: 16.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        });
                                    });
                            } else {
                                map.spawn(CustomButton::Primary.bundle(my_assets))
                                    .with_children(|button| {
                                        button.spawn(TextBundle {
                                            text: Text::from_section(
                                                format!(
                                                    "Mission {}: Level {}",
                                                    mission.name, mission.level
                                                ),
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
                        }
                    });
            }
        });
}
