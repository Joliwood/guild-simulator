use super::map_recruit_card::map_recruit_card;
use crate::{
    my_assets::MyAssets, structs::player_stats::PlayerStats,
    utils::sort_recruits_by_total_merged_stats,
};
use bevy::prelude::*;

pub fn map_recruit_list(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1400, 400),
        7,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn(Node {
            // image: my_assets.inventory_container.clone().into(),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(5.0),
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect {
                left: Val::Px(6.),
                right: Val::Px(6.),
                top: Val::Px(10.),
                bottom: Val::Px(10.),
            },
            ..default()
        })
        .with_children(|left_container| {
            let sorted_recruits =
                sort_recruits_by_total_merged_stats(player_stats.recruits.clone());

            // Barrack room > left container > recruit buttons
            for recruit in sorted_recruits.iter() {
                map_recruit_card(
                    left_container,
                    my_assets,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                );
            }
        });
}
