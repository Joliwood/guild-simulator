use super::spawn_inventory::spawn_inventory;
use crate::{enums::ColorPaletteEnum, my_assets::MyAssets, structs::player_stats::PlayerStats};
use bevy::prelude::*;

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Container for the inventory
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(400.),
                height: Val::Px(450.),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Room barrack > inventory"))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Inventory",
                TextStyle {
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 30.,
                    color: ColorPaletteEnum::DarkBrown.as_color(),
                },
            ));
            // Spawn the inventory below the buttons
            spawn_inventory(parent, player_stats, my_assets, texture_atlas_layouts);
        });
}
