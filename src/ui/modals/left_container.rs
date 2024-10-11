use crate::{
    custom_components::CustomButton,
    structs::general_structs::{PlayerStats, UniqueId},
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
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
                    .spawn(CustomButton::Primary.bundle(my_assets))
                    .insert((
                        UniqueId("assign_recruit_to_mission".to_string()),
                        recruit.clone(),
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text::from_section(
                                recruit.class.to_string(),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone().into(),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}
