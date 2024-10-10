use super::{
    armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
};
use crate::{
    enums::ColorPaletteEnum,
    structs::general_structs::{PlayerStats, SelectedRecruit},
};
use bevy::prelude::*;

pub fn recruit_infos(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_stats: &Res<PlayerStats>,
) {
    // let frame_image_handle: Handle<Image> =
    //     asset_server.load("images/rooms/barrack/recruit_infos.png");

    parent
        .spawn(ImageBundle {
            // image: frame_image_handle.into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Auto,
                height: Val::Auto,
                padding: UiRect {
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                },
                ..default()
            },
            z_index: ZIndex::Global(2),
            ..default()
        })
        .insert(Name::new("Barrack > recruit overview > recruit infos"))
        .with_children(|parent| {
            // Top container (holds Weapons and Armor buttons)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|top_container| {
                    // Left column (Weapon)
                    top_container
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|weapon_column| {
                            // Header for Weapon
                            weapon_column.spawn(TextBundle::from_section(
                                "Weapon",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            weapon_button(
                                player_stats,
                                weapon_column,
                                asset_server,
                                selected_recruit,
                                texture_atlas_layouts,
                            );
                        });

                    // Right column (Armor)
                    top_container
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|armor_column| {
                            // Header for Armor
                            armor_column.spawn(TextBundle::from_section(
                                "Armor",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            armor_button(
                                player_stats,
                                armor_column,
                                asset_server,
                                selected_recruit,
                                texture_atlas_layouts,
                            );
                        });
                });

            // Bottom container (holds Scrolls header and three buttons)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom_container| {
                    // Header for Scrolls
                    bottom_container.spawn(TextBundle::from_section(
                        "Scrolls",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));

                    // Row container for buttons
                    bottom_container
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|scrolls_row| {
                            scroll_button(
                                player_stats,
                                scrolls_row,
                                asset_server,
                                selected_recruit,
                                texture_atlas_layouts,
                                0,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                asset_server,
                                selected_recruit,
                                texture_atlas_layouts,
                                1,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                asset_server,
                                selected_recruit,
                                texture_atlas_layouts,
                                2,
                            );
                        });
                });
        });
}
