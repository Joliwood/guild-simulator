use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::get_item_atlas_path,
    structs::{
        equipments::ItemEnum, general_structs::UniqueId, player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
    },
    utils::{
        get_item_image_atlas_index,
        get_layout,
        // get_item_tooltip_description
    },
};
use bevy::prelude::*;
// use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn scroll_button(
    player_stats: &Res<PlayerStats>,
    scrolls_row: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    scroll_index: u8,
) {
    let recruit_id = selected_recruit_for_equipment.get_id();

    if recruit_id.is_none() {
        // Empty scroll button
        scrolls_row
            .spawn((
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
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
        return;
    }

    let recruit = player_stats.get_recruit_by_id(recruit_id.unwrap()).unwrap();
    let recruit_inventory = recruit.recruit_inventory;
    let recruit_scrolls = recruit_inventory.scrolls;
    let recruit_scroll = recruit_scrolls.get(scroll_index as usize);

    if let Some(recruit_scroll) = recruit_scroll {
        let item = ItemEnum::Scroll(recruit_scroll.clone(), 1);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_layout = get_layout(TextureAtlasLayoutEnum::Item(&item));
        // let tooltip_text = get_item_tooltip_description(&item);
        let item_atlas_path = get_item_atlas_path(&item);

        // Scroll button
        scrolls_row
            .spawn((
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
                UiImage::from_atlas_image(
                    my_assets.load(item_atlas_path),
                    TextureAtlas {
                        index: item_image_atlas_index.into(),
                        layout: texture_atlas_layouts.add(item_layout),
                    },
                ),
                // Tooltip::cursor(tooltip_text.to_string())
                //     .with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
    } else {
        // Empty scroll button
        scrolls_row
            .spawn((
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
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
    }
}
