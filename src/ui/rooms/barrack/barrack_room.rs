use super::{
    inventory::spawn_right_container::spawn_right_container,
    recruit_overview_folder::recruit_overview::recruit_overview,
    recruits_list_folder::recruits_list::spawn_left_container,
};
use crate::{
    structs::{
        player_stats::PlayerStats, recruits::SelectedRecruitForEquipment,
        trigger_structs::ResetRoomTrigger,
    },
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::node_container_style},
};
use bevy::prelude::*;

pub fn spawn_room_barrack(
    my_assets: &Res<MyAssets>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Auto),
                ..node_container_style()
            },
            z_index: ZIndex::Global(-1),
            ..default()
        })
        .insert(Name::new("Room barrack"))
        .insert(ResetRoomTrigger)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    height: Val::Vh(100.),
                    ..default()
                },
                image: my_assets.barrack_background.clone().into(),
                ..default()
            });

            spawn_left_container(parent, my_assets, player_stats, texture_atlas_layouts);
            recruit_overview(
                player_stats,
                parent,
                my_assets,
                selected_recruit_for_equipment,
                texture_atlas_layouts,
            );
            spawn_right_container(parent, my_assets, player_stats, texture_atlas_layouts);
        });
}
