use crate::{
    structs::{
        equipments::Item,
        general_structs::{RecruitStats, UniqueId},
    },
    utils::{
        get_item_atlas_path, get_item_image_atlas_index, get_item_layout,
        get_item_tooltip_description,
    },
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn weapon_button(
    top_container: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    recruit_stats: &RecruitStats,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle_empty_slot: Handle<Image> =
        asset_server.load("images/equipments/empty_inventory_slot.png");

    let recruit_stats_inventory = recruit_stats.recruit_inventory.clone();
    let recruit_stats_weapon = recruit_stats_inventory.weapon;
    if let Some(recruit_stats_weapon) = recruit_stats_weapon {
        let item = Item::Weapon(recruit_stats_weapon);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_atlas_path = get_item_atlas_path(&item);
        let layout = get_item_layout(&item);
        let tooltip_text = get_item_tooltip_description(&item);

        top_container
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        border: UiRect::all(Val::Px(3.)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    image: asset_server.load(item_atlas_path).clone().into(),
                    ..default()
                },
                TextureAtlas {
                    index: item_image_atlas_index.into(),
                    layout: texture_atlas_layouts.add(layout),
                },
                Tooltip::cursor(tooltip_text.to_string())
                    .with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
    } else {
        // Empty weapon button
        top_container
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                image: texture_handle_empty_slot.clone().into(),
                ..default()
            })
            .insert(UniqueId("item_in_inventory".to_string()));
    }
}
