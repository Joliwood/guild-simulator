use crate::structs::{general_structs::SelectedRecruit, trigger_structs::SelectedRecruitTrigger};
use bevy::prelude::*;

use super::{recruit_frame::recruit_frame, recruit_infos::recruit_infos};

pub fn recruit_overview(
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
                // width: Val::Px(200.),
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
        // Insert childrens
        .with_children(|parent| {
            // 1 - Recruit frame
            recruit_frame(
                parent,
                asset_server,
                selected_recruit,
                texture_atlas_layouts,
            );
            // 2 - Recruit infos
            recruit_infos(
                parent,
                asset_server,
                selected_recruit,
                texture_atlas_layouts,
            )
        });
}
