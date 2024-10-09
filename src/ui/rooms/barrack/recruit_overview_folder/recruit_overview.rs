use super::{recruit_frame::recruit_frame, recruit_infos_folder::recruit_infos::recruit_infos};
use crate::structs::general_structs::{PlayerStats, SelectedRecruit};
use bevy::prelude::*;

pub fn recruit_overview(
    player_stats: &Res<PlayerStats>,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
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
                width: Val::Auto,
                margin: UiRect {
                    left: Val::Px(50.),
                    right: Val::Px(50.),
                    top: Val::Px(35.),
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
                asset_server,
                selected_recruit,
                texture_atlas_layouts,
            );
            recruit_infos(
                parent,
                asset_server,
                selected_recruit,
                texture_atlas_layouts,
                player_stats,
            )
        });
}
