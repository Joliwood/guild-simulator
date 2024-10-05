use bevy::prelude::*;

use crate::structs::trigger_structs::MissionReport;

pub fn mission_report_documents(
    asset_server: &Res<AssetServer>,
    elements_on_desk: &mut ChildBuilder,
) {
    let mission_report_documents_image: Handle<Image> =
        asset_server.load("images/rooms/office/mission_notification_document.png");

    elements_on_desk
        .spawn(ImageBundle {
            image: mission_report_documents_image.into(),
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                top: Val::Px(50.),
                width: Val::Px(150.),
                height: Val::Px(133. + 66.5),
                ..default()
            },
            ..default()
        })
        .insert(MissionReport)
        .insert(Interaction::default());
}
