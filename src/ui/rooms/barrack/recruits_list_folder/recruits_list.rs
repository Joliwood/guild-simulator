use super::recruit_card::recruit_card;
use crate::{my_assets::MyAssets, structs::player_stats::PlayerStats};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
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
            display: Display::Flex,
            align_self: AlignSelf::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                top: Val::Px(30.),
                bottom: Val::Px(25.),
                left: Val::Px(25.),
                right: Val::Px(25.),
            },
            row_gap: Val::Px(5.0),
            align_items: AlignItems::Center,
            width: Val::Px(400.),
            height: Val::Px(450.),
            ..default()
        })
        .insert(Name::new("Barrack > Recruits list"))
        .with_children(|left_container| {
            // Background image
            left_container.spawn((
                UiImage {
                    image: my_assets.inventory_container.clone().into(),
                    ..default()
                },
                Node {
                    display: Display::Flex,
                    top: Val::Px(0.),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));

            // Barrack room > left container > recruit buttons
            for recruit in player_stats.recruits.iter() {
                recruit_card(
                    left_container,
                    my_assets,
                    player_stats,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                    texture_atlas_layouts,
                );
            }
        });
}
