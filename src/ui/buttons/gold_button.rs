use crate::structs::UniqueId;
use crate::systems::systems_constants::NORMAL_BUTTON;
use bevy::prelude::*;

pub fn gold_button(asset_server: &Res<AssetServer>, commands: &mut ChildBuilder) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            image: UiImage::default().with_color(NORMAL_BUTTON),
            ..default()
        })
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
