use crate::structs::trigger_structs::SleepButtonTrigger;
use bevy::prelude::*;

pub fn set_of_keys(asset_server: &Res<AssetServer>, elements_on_desk: &mut ChildBuilder) {
    let mission_report_documents_image: Handle<Image> =
        asset_server.load("images/rooms/office/set_of_keys.png");

    elements_on_desk
        .spawn((ButtonBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(280.),
                right: Val::Px(50.),
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            ..default()
        },))
        .insert(SleepButtonTrigger)
        .with_children(|sleep_button| {
            sleep_button.spawn(ImageBundle {
                image: mission_report_documents_image.into(),
                style: Style {
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            });
        });
}
