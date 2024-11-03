use bevy::prelude::*;

use crate::{enums::ColorPaletteEnum, my_assets::MyAssets};

pub fn recruit_endurance(
    stats_container: &mut ChildBuilder,
    recruit_endurance: u32,
    additional_endurance: u32,
    my_assets: &Res<MyAssets>,
) {
    let base_font: Handle<Font> = my_assets.fira_sans_bold.clone();

    // let base_endurance_text = TextSection {
    //     value: format!("END: {}", recruit_endurance),
    //     style: TextFont {
    //         font: base_font.clone(),
    //         font_size: 14.0,
    //         color: ColorPaletteEnum::DarkBrown.as_color(),
    //     },
    // };

    // let additional_endurance_text = if additional_endurance > 0 {
    //     TextSection {
    //         value: format!(" (+{})", additional_endurance),
    //         style: TextFont {
    //             font: base_font.clone(),
    //             font_size: 14.0,
    //             color: Color::srgb(0.0, 107.0 / 255.0, 29.0 / 255.0),
    //         },
    //     }
    // } else {
    //     // Empty section if additional endurance is zero or less
    //     TextSection {
    //         value: String::new(),
    //         style: TextFont {
    //             font: base_font,
    //             font_size: 14.0,
    //             color: Color::NONE,
    //         },
    //     }
    // };

    stats_container.spawn((
        Text::new(format!(
            "END: {} (+{})",
            recruit_endurance, additional_endurance
        )),
        // text: Text::from_sections([base_endurance_text, additional_endurance_text]),
    ));
}
