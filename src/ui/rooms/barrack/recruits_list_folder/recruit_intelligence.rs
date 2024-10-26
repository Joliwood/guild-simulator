use bevy::prelude::*;

use crate::{enums::ColorPaletteEnum, my_assets::MyAssets};

pub fn recruit_intelligence(
    stats_container: &mut ChildBuilder,
    recruit_intelligence: u32,
    additional_intelligence: u32,
    my_assets: &Res<MyAssets>,
) {
    let base_font: Handle<Font> = my_assets.fira_sans_bold.clone();

    let base_intelligence_text = TextSection {
        value: format!("INT: {}", recruit_intelligence),
        style: TextStyle {
            font: base_font.clone(),
            font_size: 14.0,
            color: ColorPaletteEnum::DarkBrown.as_color(),
        },
    };

    let additional_intelligence_text = if additional_intelligence > 0 {
        TextSection {
            value: format!(" (+{})", additional_intelligence),
            style: TextStyle {
                font: base_font.clone(),
                font_size: 14.0,
                color: Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0),
            },
        }
    } else {
        // Empty section if additional intelligence is zero or less
        TextSection {
            value: String::new(),
            style: TextStyle {
                font: base_font,
                font_size: 14.0,
                color: Color::NONE,
            },
        }
    };

    stats_container.spawn(TextBundle {
        text: Text::from_sections([base_intelligence_text, additional_intelligence_text]),
        ..default()
    });
}
