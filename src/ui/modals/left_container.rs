use crate::{
    custom_components::CustomButton,
    structs::general_structs::{PlayerStats, UniqueId},
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
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                width: Val::Percent(30.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(Color::WHITE),
            ..default()
        })
        .with_children(|parent| {
            // Recruits
            for recruit in player_stats.recruits.iter() {
                parent
                    .spawn(CustomButton::Primary.bundle(asset_server, image_assets))
                    .insert((
                        UniqueId(format!("assign_recruit_to_mission")),
                        recruit.clone(),
                    ))
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
