use crate::{
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, SelectedRecruit, UniqueId},
    },
    ui::interface::gold_counter::MyAssets,
    utils::{get_item_image_atlas_index, get_item_layout, get_item_tooltip_description},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn weapon_button(
    player_stats: &Res<PlayerStats>,
    weapon_column: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_id = selected_recruit.get_id();

    if recruit_id.is_none() {
        // Empty weapon button
        weapon_column
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
                image: my_assets.empty_inventory_slot.clone().into(),
                ..default()
            })
            .insert(UniqueId("item_in_inventory".to_string()));
        return;
    }

    let recruit = player_stats.get_recruit_by_id(recruit_id.unwrap()).unwrap();

    let recruit_inventory = recruit.recruit_inventory;
    let recruit_weapon = recruit_inventory.weapon;
    if let Some(recruit_weapon) = recruit_weapon {
        let item = Item::Weapon(recruit_weapon);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let layout = get_item_layout(&item);
        let tooltip_text = get_item_tooltip_description(&item);

        // Weapon button
        weapon_column
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(60.),
                        height: Val::Px(60.),
                        border: UiRect::all(Val::Px(3.)),
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    image: my_assets.get_item_atlas_path(&item).clone().into(),
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    ..default()
                },
                TextureAtlas {
                    index: item_image_atlas_index.into(),
                    layout: texture_atlas_layouts.add(layout),
                },
                Tooltip::cursor(tooltip_text).with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
    } else {
        // Empty weapon button
        weapon_column
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
                image: my_assets.empty_inventory_slot.clone().into(),
                ..default()
            })
            .insert(UniqueId("item_in_inventory".to_string()));
    }
}
