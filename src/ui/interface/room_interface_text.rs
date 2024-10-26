use crate::{
    my_assets::MyAssets,
    structs::{player_stats::PlayerStats, trigger_structs::PlayerStatsRoomTrigger},
    ui::{styles::containers_styles::room_interface_text_style, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

pub fn room_interface_text(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
) {
    commands
        // Container
        .spawn(NodeBundle {
            style: room_interface_text_style(),
            background_color: BackgroundColor(WOOD_COLOR),
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
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(PlayerStatsRoomTrigger);
        });
}
