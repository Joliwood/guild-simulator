use super::{left_hud::left_hud, right_hud::right_hud};
use crate::{
    enums::RoomEnum,
    my_assets::MyAssets,
    structs::{player_stats::PlayerStats, trigger_structs::RoomButtonTrigger},
};
use bevy::prelude::*;

pub fn hud(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let hud_icons_layout = TextureAtlasLayout::from_grid(
        UVec2::new(4000, 500),
        8,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let hud_icons_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(hud_icons_layout);

    commands
        // Main Container
        .spawn(ImageBundle {
            image: my_assets.hud.clone().into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                display: Display::Flex,
                ..default()
            },
            z_index: ZIndex::Global(3),
            ..default()
        })
        .insert(Name::new("HUD"))
        // Left Container
        .with_children(|parent| {
            left_hud(
                parent,
                &my_assets,
                &player_stats,
                &hud_icons_texture_atlas_layout,
            );

            // Middle Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        height: Val::Px(35.0),
                        width: Val::Px(165.0),
                        margin: UiRect {
                            right: Val::Px(2.),
                            ..default()
                        },
                        column_gap: Val::Px(7.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Middle Container"))
                .with_children(|middle_container| {
                    middle_container
                        .spawn((
                            ButtonBundle {
                                image: my_assets.hud_icon_atlas.clone().into(),
                                style: Style {
                                    width: Val::Px(30.),
                                    height: Val::Px(30.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                index: 4,
                                layout: hud_icons_texture_atlas_layout.clone(),
                            },
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::CommandRoom));

                    middle_container
                        .spawn((
                            ButtonBundle {
                                image: my_assets.hud_icon_atlas.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                index: 1,
                                layout: hud_icons_texture_atlas_layout.clone(),
                            },
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Office));

                    middle_container
                        .spawn((
                            ButtonBundle {
                                image: my_assets.hud_icon_atlas.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                index: 2,
                                layout: hud_icons_texture_atlas_layout.clone(),
                            },
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Barrack));
                });
            right_hud(
                parent,
                &my_assets,
                &player_stats,
                hud_icons_texture_atlas_layout,
            );
        });
}
