use crate::{enums::ColorPaletteEnum, my_assets::FONT_FIRA, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_magical_power(
    stats_container: &mut ChildBuilder,
    recruit: &RecruitStats,
    additional_magical_power: u32,
    my_assets: &Res<AssetServer>,
) {
    stats_container
        .spawn(Node::default())
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .with_children(|node| {
            node.spawn((
                // PP : Physical power
                // MP : Magical power
                Text::new(format!("MP:{}", recruit.magical_power)),
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

            if additional_magical_power > 0 {
                node.spawn((
                    Text::new(format!("(+{})", additional_magical_power)),
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
