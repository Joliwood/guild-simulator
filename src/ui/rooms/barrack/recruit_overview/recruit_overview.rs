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
                row_gap: Val::Px(20.),
                width: Val::Px(200.),
                margin: UiRect {
                    left: Val::Px(50.),
                    right: Val::Px(50.),
                    top: Val::Px(50.),
                    bottom: Val::ZERO,
                },
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
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
