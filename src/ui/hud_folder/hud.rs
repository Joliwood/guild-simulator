use super::{left_hud::left_hud, right_hud::right_hud, sleep_button::sleep_button};
use crate::{
    enums::{RoomEnum, TextureAtlasLayoutEnum},
    structs::{player_stats::PlayerStats, trigger_structs::RoomButtonTrigger},
    utils::get_layout,
};
use bevy::prelude::*;
// use pyri_tooltip::Tooltip;

pub fn hud(
    my_assets: Res<AssetServer>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let hud_icons_layout = get_layout(TextureAtlasLayoutEnum::HudIcon);
    let hud_icons_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(hud_icons_layout);

    commands
        // Main Container
        .spawn((
            UiImage {
                image: my_assets.load("images/hud/hud4.png"),
                ..default()
            },
            Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                display: Display::Flex,
                ..default()
            },
            GlobalZIndex(3),
        ))
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
                .spawn(Node {
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
                })
                .insert(Name::new("Middle Container"))
                .with_children(|middle_container| {
                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.),
                                height: Val::Px(30.),
                                ..default()
                            },
                            UiImage::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 4,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            // Tooltip::cursor("Go to the command room\n\nShortcut: press 'S'"),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::CommandRoom));

                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.),
                                ..default()
                            },
                            UiImage::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 1,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            // Tooltip::cursor("Go to the office room\n\nShortcut: press 'W/Z'"),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Office));

                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.),
                                ..default()
                            },
                            UiImage::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 2,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            // Tooltip::cursor("Go to the barrack room\n\nShortcut: press 'D'"),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Barrack));
                });

            right_hud(
                parent,
                &my_assets,
                &player_stats,
                &hud_icons_texture_atlas_layout,
            );
        });

    sleep_button(
        &mut commands,
        &my_assets,
        &player_stats,
        &mut texture_atlas_layouts,
    );
}
