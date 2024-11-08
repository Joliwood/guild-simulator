use crate::{enums::ColorPaletteEnum, my_assets::FONT_FIRA};
use bevy::prelude::*;

pub fn recruit_strength(
    stats_container: &mut ChildBuilder,
    recruit_strength: u32,
    additional_strength: u32,
    my_assets: &Res<AssetServer>,
) {
    stats_container
        .spawn(Node::default())
        .with_children(|node| {
            node.spawn((
                Text::new(format!("STR: {}", recruit_strength)),
                TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 12.0,
                    ..default()
                },
            ));

            if additional_strength > 0 {
                node.spawn((
                    Text::new(format!(" (+{})", additional_strength)),
                    TextColor(Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0)),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 12.0,
                        ..default()
                    },
                ));
            }
        });
}
