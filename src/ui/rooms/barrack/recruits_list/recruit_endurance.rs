use bevy::prelude::*;

use crate::enums::ColorPaletteEnum;

pub fn recruit_endurance(
    stats_container: &mut ChildBuilder,
    recruit_endurance: u32,
    additional_endurance: u32,
    asset_server: &Res<AssetServer>,
) {
    let base_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let base_endurance_text = TextSection {
        value: format!("END: {}", recruit_endurance),
        style: TextStyle {
            font: base_font.clone(),
            font_size: 14.0,
            color: ColorPaletteEnum::DarkBrown.as_color(),
        },
    };

    let additional_endurance_text = if additional_endurance > 0 {
        TextSection {
            value: format!(" (+{})", additional_endurance),
            style: TextStyle {
                font: base_font.clone(),
                font_size: 14.0,
                color: Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0), // Using your custom green color
            },
        }
    } else {
        // Empty section if additional endurance is zero or less
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
        text: Text::from_sections([base_endurance_text, additional_endurance_text]),
        ..default()
    });
}
