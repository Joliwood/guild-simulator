use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::get_item_atlas_path,
    structs::{
        equipments::ItemEnum, player_stats::PlayerStats, recruits::SelectedRecruitForEquipment,
        trigger_structs::ItemInInventoryTrigger,
    },
    utils::{get_item_image_atlas_index, get_item_tooltip_description, get_layout},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn armor_button(
    player_stats: &Res<PlayerStats>,
    armor_column: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_id = selected_recruit_for_equipment.get_id();

    if recruit_id.is_none() {
        // Empty armor button
        armor_column.spawn((
            Button,
            Node {
                width: Val::Px(60.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
            UiImage {
                image: my_assets.load("images/equipments/empty_inventory_slot.png"),
                ..default()
            },
            ItemInInventoryTrigger(None),
        ));
        return;
    }

    let recruit = player_stats.get_recruit_by_id(recruit_id.unwrap()).unwrap();
    let recruit_inventory = recruit.recruit_inventory;
    let recruit_armor = recruit_inventory.armor;

    if let Some(recruit_armor) = recruit_armor {
        let item = ItemEnum::Armor(recruit_armor);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_layout = get_layout(TextureAtlasLayoutEnum::Item(&item));
        let tooltip_text = get_item_tooltip_description(&item);
        let item_atlas_path = get_item_atlas_path(&item);

        // Armor button
        armor_column.spawn((
            Button,
            Node {
                width: Val::Px(60.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            UiImage::from_atlas_image(
                my_assets.load(item_atlas_path),
                TextureAtlas {
                    index: item_image_atlas_index.into(),
                    layout: texture_atlas_layouts.add(item_layout),
                },
            ),
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
            ItemInInventoryTrigger(None),
            Tooltip::cursor(t!(tooltip_text).to_string())
                .with_activation(TooltipActivation::IMMEDIATE),
        ));
    } else {
        // Empty armor button
        armor_column.spawn((
            Button,
            Node {
                width: Val::Px(60.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            UiImage {
                image: my_assets.load("images/equipments/empty_inventory_slot.png"),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
            ItemInInventoryTrigger(None),
        ));
    }
}
