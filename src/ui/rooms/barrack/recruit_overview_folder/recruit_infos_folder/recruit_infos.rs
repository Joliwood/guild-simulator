use super::{
    armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
};
use crate::{
    enums::ColorPaletteEnum,
    my_assets::MyAssets,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment},
};
use bevy::prelude::*;

pub fn recruit_infos(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_stats: &Res<PlayerStats>,
) {
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                height: Val::Auto,
                padding: UiRect {
                    top: Val::Px(15.),
                    bottom: Val::Px(15.),
                    left: Val::Px(20.),
                    right: Val::Px(20.),
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
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            weapon_button(
                                player_stats,
                                weapon_column,
                                my_assets,
                                selected_recruit_for_equipment,
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
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 20.0,
                                    color: ColorPaletteEnum::DarkBrown.as_color(),
                                },
                            ));

                            armor_button(
                                player_stats,
                                armor_column,
                                my_assets,
                                selected_recruit_for_equipment,
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
                            font: my_assets.fira_sans_bold.clone(),
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
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                0,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                1,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                2,
                            );
                        });
                });
        });
}
