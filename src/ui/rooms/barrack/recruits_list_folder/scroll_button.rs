use crate::{
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, RecruitStats, UniqueId},
    },
    ui::interface::gold_counter::MyAssets,
    utils::{
        get_item_atlas_path, get_item_image_atlas_index, get_item_layout,
        get_item_tooltip_description,
    },
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn scroll_button(
    player_stats: &Res<PlayerStats>,
    scrolls_row: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    recruit: &RecruitStats,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    scroll_index: u8,
) {
    // let texture_handle_empty_slot: Handle<Image> =
    //     my_assets.load("images/equipments/empty_inventory_slot.png");

    let recruit_id = recruit.id;

    let recruit = player_stats.get_recruit_by_id(recruit_id).unwrap();
    let recruit_inventory = recruit.recruit_inventory;
    let recruit_scrolls = recruit_inventory.scrolls;
    let recruit_scroll = recruit_scrolls.get(scroll_index as usize);

    if let Some(recruit_scroll) = recruit_scroll {
        let item = Item::Scroll(recruit_scroll.clone(), 1);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        // WIP
        // let item_atlas_path = get_item_atlas_path(&item);
        let layout = get_item_layout(&item);
        let tooltip_text = get_item_tooltip_description(&item);

        // Scroll button
        scrolls_row
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        border: UiRect::all(Val::Px(3.)),
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
                Tooltip::cursor(tooltip_text.to_string())
                    .with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(UniqueId("item_in_inventory".to_string()));
    } else {
        // Empty scroll button
        scrolls_row
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(3.)),
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
