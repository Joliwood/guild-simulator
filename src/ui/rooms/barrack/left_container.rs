use crate::{
    structs::{
        general_structs::{PlayerStats, UniqueId},
        trigger_structs::SelectedRecruitTrigger,
    },
    styles::CustomButton,
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    image_assets: &Res<MyAssets>,
) {
    let image_handle: Handle<Image> = asset_server.load("images/rooms/barrack/barrack.png");

    parent
        // Left container
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(25.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Barrack > left container"))
        .with_children(|left_container| {
            // Background image
            left_container.spawn(ImageBundle {
                image: image_handle.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            // Barrack room > left container > recruit buttons
            for recruit in player_stats.recruits.iter() {
                left_container
                    .spawn(CustomButton::Primary.bundle(&asset_server, image_assets))
                    .insert((
                        UniqueId(format!("recruit_button_{}", recruit.id)),
                        SelectedRecruitTrigger,
                    )) // Use recruit.id here
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text::from_section(
                                &recruit.class.to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}
