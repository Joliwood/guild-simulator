use bevy::prelude::*;

use crate::{
    custom_components::CustomButton, structs::general_structs::UniqueId,
    ui::interface::gold_counter::MyAssets,
};

pub fn loots_and_start(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    // texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Loots (Text / Loot Icons)
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
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
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });

            //     // Loot icons
            //     parent.spawn(ImageBundle {
            //         image: UiImage(my_assets.loot_image.clone()),
            //         style: Style {
            //             width: Val::Px(40.0),
            //             height: Val::Px(40.0),
            //             ..default()
            //         },
            //         ..default()
            //     });
            // });

            // // Sign the mission button
            // parent
            //     .spawn(ButtonBundle {
            //         style: Style {
            //             margin: UiRect::all(Val::Px(10.0)),
            //             width: Val::Percent(20.0),
            //             height: Val::Px(50.0),
            //             ..default()
            //         },
            //         image: my_assets._play.clone().into(),
            //         ..default()
            //     })
            //     .insert(UniqueId("sign_mission_order".to_string()));

            // Button inside the middle container
            parent
                .spawn(CustomButton::MissionStart.bundle(my_assets))
                .insert(UniqueId("start_mission".to_string()))
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
                });
        });
}
