use crate::{
    my_assets::MyAssets,
    structs::{player_stats::PlayerStats, trigger_structs::PlayerStatsRoomTrigger},
};
use bevy::prelude::*;

pub fn hud(my_assets: Res<MyAssets>, mut commands: Commands, player_stats: Res<PlayerStats>) {
    commands
        // Container
        .spawn(ImageBundle {
            image: my_assets.hud.clone().into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                ..default()
            },
            z_index: ZIndex::Global(3),
            ..default()
        })
        .insert(Name::new("Room interface text"))
        // Room text
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(TextBundle {
                    text: Text::from_section(
                        format! {"{gold_counter} | Guild level : {guild_level}",
                        gold_counter = player_stats.golds,
                        guild_level = player_stats.guild_level},
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(PlayerStatsRoomTrigger);
        });
}
