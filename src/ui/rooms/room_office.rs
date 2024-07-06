use crate::{
    structs::{ResetRoomTrigger, UniqueId},
    systems::systems_constants::NORMAL_BUTTON,
};
use bevy::prelude::*;

pub fn room_office(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    let imager_handler: Handle<Image> = asset_server.load("images/office.png");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: imager_handler.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            });
        })
        // Menu button node
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
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
        });
}
