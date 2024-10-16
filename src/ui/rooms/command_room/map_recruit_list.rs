use super::map_recruit_card::map_recruit_card;
use crate::{
    structs::{missions::SelectedMission, player_stats::PlayerStats},
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

pub fn map_recruit_list(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    selected_mission: &mut ResMut<SelectedMission>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(40.),
                padding: UiRect {
                    left: Val::Px(6.),
                    right: Val::Px(6.),
                    top: Val::Px(4.),
                    bottom: Val::Px(4.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|left_container| {
            // Barrack room > left container > recruit buttons
            for recruit in player_stats.recruits.iter() {
                map_recruit_card(
                    left_container,
                    my_assets,
                    player_stats,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                    texture_atlas_layouts,
                    selected_mission,
                );
            }
        });
}
