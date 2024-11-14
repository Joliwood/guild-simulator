use super::recruit_card::recruit_card;
use crate::{enums::TextureAtlasLayoutEnum, structs::player_stats::PlayerStats, utils::get_layout};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn((Node {
            display: Display::Flex,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            width: Val::Px(400.),
            height: Val::Px(450.),
            ..default()
        },))
        .insert(Name::new("Barrack > Recruits list"))
        .with_children(|left_container| {
            // Background image
            left_container
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/barrack/inventory_container.png"),
                        ..default()
                    },
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        top: Val::Px(0.),
                        row_gap: Val::Px(5.0),
                        padding: UiRect {
                            top: Val::Px(30.),
                            bottom: Val::Px(25.),
                            left: Val::Px(25.),
                            right: Val::Px(25.),
                        },
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Barrack room > left container > recruit buttons
                    for recruit in player_stats.recruits.iter() {
                        recruit_card(
                            parent,
                            my_assets,
                            player_stats,
                            recruit,
                            recruit_texture_atlas_layout.clone(),
                            texture_atlas_layouts,
                        );
                    }
                });
        });
}
