use super::{recruit_frame::recruit_frame, recruit_infos_folder::recruit_infos::recruit_infos};
use crate::{
    my_assets::MyAssets,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment},
};
use bevy::prelude::*;

pub fn recruit_overview(
    player_stats: &Res<PlayerStats>,
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Right container: recruit info
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.),
                margin: UiRect {
                    left: Val::Px(30.),
                    right: Val::Px(30.),
                    top: Val::Px(60.),
                    bottom: Val::ZERO,
                },
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Barrack > recruit overview"))
        .with_children(|parent| {
            recruit_frame(
                parent,
                my_assets,
                selected_recruit_for_equipment,
                texture_atlas_layouts,
            );
            recruit_infos(
                parent,
                my_assets,
                selected_recruit_for_equipment,
                texture_atlas_layouts,
                player_stats,
            )
        });
}
