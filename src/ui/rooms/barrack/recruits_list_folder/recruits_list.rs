use super::recruit_card::recruit_card;
use crate::{structs::general_structs::PlayerStats, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // let inventory_container_image_handle: Handle<Image> =
    //     my_assets.load("images/rooms/barrack/inventory_container.png");

    // let recruit_image_handle: Handle<Image> =
    //     my_assets.load("images/recruits/recruit_picture_atlas.png");

    let recruit_image_handle: Handle<Image> = my_assets.recruit_picture_atlas.clone().into();

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
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Px(400.),
                height: Val::Px(450.),
                padding: UiRect::all(Val::Px(15.0)),
                // overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Barrack > Recruits list"))
        .with_children(|left_container| {
            // Background image
            left_container.spawn(ImageBundle {
                image: my_assets.inventory_container.clone().into(),
                style: Style {
                    display: Display::Flex,
                    top: Val::Px(0.),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            // Barrack room > left container > recruit buttons
            for recruit in player_stats.recruits.iter() {
                recruit_card(
                    left_container,
                    my_assets,
                    player_stats,
                    recruit,
                    // recruit_image_handle,
                    recruit_texture_atlas_layout.clone(),
                    texture_atlas_layouts,
                );
            }
        });
}
