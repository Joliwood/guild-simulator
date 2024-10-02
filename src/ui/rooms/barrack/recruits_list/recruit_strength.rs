use bevy::prelude::*;

use crate::enums::ColorPaletteEnum;

pub fn recruit_strength(
    stats_container: &mut ChildBuilder,
    recruit_strength: u32,
    additional_strength: u32,
    asset_server: &Res<AssetServer>,
) {
    let base_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let base_strength_text = TextSection {
        value: format!("STR: {}", recruit_strength),
        style: TextStyle {
            font: base_font.clone(),
            font_size: 14.0,
            color: ColorPaletteEnum::DarkBrown.as_color(),
        },
    };

    let additional_strength_text = if additional_strength > 0 {
        TextSection {
            value: format!(" (+{})", additional_strength),
            style: TextStyle {
                font: base_font.clone(),
                font_size: 14.0,
                color: Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0), // Using your custom green color
            },
        }
    } else {
        // Empty section if additional strength is zero or less
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
        text: Text::from_sections([base_strength_text, additional_strength_text]),
        ..default()
    });
}
