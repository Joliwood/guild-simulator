use crate::structs::UniqueId;
use crate::styles::CustomButton;
use bevy::prelude::*;

pub fn gold_button(asset_server: &Res<AssetServer>, commands: &mut ChildBuilder) {
    commands
        .spawn(CustomButton::GoldButton.bundle(&asset_server))
        .insert(Name::new("Gold button"))
        .insert(UniqueId("menu_button_id".to_string()))
        // Text inside the button
        .with_children(|settings_button| {
            settings_button.spawn(TextBundle::from_section(
                "X",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
