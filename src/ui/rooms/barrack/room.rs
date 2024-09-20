use crate::{
    structs::{
        general_structs::{PlayerStats, SelectedRecruit},
        trigger_structs::{PlayerStatsRecruitsTrigger, ResetRoomTrigger},
    },
    ui::{
        interface::gold_counter::MyAssets,
        rooms::barrack::{
            left_container::spawn_left_container, middle_container::spawn_middle_container,
            right_container::spawn_right_container,
        },
        styles::node_container_style::node_container_style,
    },
};
use bevy::prelude::*;

pub fn spawn_room_barrack(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
    selected_recruit: &Res<SelectedRecruit>,
    image_assets: &Res<MyAssets>,
) {
    info!("Selected recruit: {:?}", selected_recruit.0);

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Auto),
                height: Val::Percent(100.0),
                ..node_container_style()
            },
            z_index: ZIndex::Global(-1),
            ..default()
        })
        .insert(Name::new("Room barrack"))
        .insert(ResetRoomTrigger)
        .insert(PlayerStatsRecruitsTrigger)
        // WIP - Spawn the left container
        .with_children(|parent| {
            spawn_left_container(parent, asset_server, player_stats, image_assets);
            spawn_middle_container(parent, asset_server, selected_recruit);
            spawn_right_container(parent, asset_server, player_stats);
        });
}
