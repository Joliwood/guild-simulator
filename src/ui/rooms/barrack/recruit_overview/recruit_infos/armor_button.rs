use crate::{
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, SelectedRecruit, UniqueId},
    },
    utils::{
        get_item_atlas_path, get_item_image_atlas_index, get_item_layout,
        get_item_tooltip_description,
    },
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn armor_button(
    player_stats: &Res<PlayerStats>,
    armor_column: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle_empty_slot: Handle<Image> =
        asset_server.load("images/equipments/empty_inventory_slot.png");

    let recruit_id = selected_recruit.get_id();

    if recruit_id.is_none() {
        // Empty armor button
        armor_column
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                image: texture_handle_empty_slot.clone().into(),
                ..default()
            })
            .insert(UniqueId(format!("item_in_inventory")));
        return;
    }

    let recruit = player_stats.get_recruit_by_id(recruit_id.unwrap()).unwrap();
    let recruit_inventory = recruit.recruit_inventory;
    let recruit_armor = recruit_inventory.armor;

    if let Some(recruit_armor) = recruit_armor {
        let item = Item::Armor(recruit_armor);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_atlas_path = get_item_atlas_path(&item);
        let layout = get_item_layout(&item);
        let tooltip_text = get_item_tooltip_description(&item);

        // Armor button
        armor_column
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(60.),
                        height: Val::Px(60.),
                        border: UiRect::all(Val::Px(3.)),
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    image: asset_server.load(item_atlas_path).clone().into(),
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    ..default()
                },
                TextureAtlas {
                    index: item_image_atlas_index.into(),
                    layout: texture_atlas_layouts.add(layout),
                },
                Tooltip::cursor(tooltip_text.to_string())
                    .with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(UniqueId(format!("item_in_inventory")));
    } else {
        // Empty armor button
        armor_column
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                image: texture_handle_empty_slot.clone().into(),
                ..default()
            })
            .insert(UniqueId(format!("item_in_inventory")));
    }
}
