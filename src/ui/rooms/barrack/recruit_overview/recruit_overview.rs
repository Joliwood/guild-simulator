use crate::structs::{general_structs::SelectedRecruit, trigger_structs::SelectedRecruitTrigger};
use bevy::prelude::*;

use super::recruit_frame::recruit_frame;

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
                width: Val::Px(250.0),
                margin: UiRect {
                    left: Val::Px(50.0),
                    right: Val::Px(50.0),
                    top: Val::ZERO,
                    bottom: Val::ZERO,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Barrack > right container"))
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
        });
}
