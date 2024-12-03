use crate::{enums::ColorPaletteEnum, my_assets::FONT_FIRA, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_attack(
    stats_container: &mut ChildBuilder,
    recruit: &RecruitStats,
    additionnal_attack_from_items: u32,
    my_assets: &Res<AssetServer>,
) {
    // let equipment_stats_multiplicator = recruit
    //     .recruit_inventory
    //     .get_equipment_stats_multiplicator_from_scroll_bonus();

    stats_container
        .spawn(Node::default())
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .with_children(|node| {
            node.spawn((
                Text::new(format!("ATT:{}", recruit.attack)),
                TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 12.0,
                    ..default()
                },
            ))
            .insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            });

            if additionnal_attack_from_items > 0 {
                node.spawn((
                    Text::new(format!("(+{})", additionnal_attack_from_items)),
                    TextColor(Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0)),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 12.0,
                        ..default()
                    },
                ))
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                });
            }
        });
}
