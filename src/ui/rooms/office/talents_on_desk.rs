use bevy::prelude::*;

pub fn talents_on_desk(asset_server: &Res<AssetServer>, elements_on_desk: &mut ChildBuilder) {
    let mission_report_documents_image: Handle<Image> =
        asset_server.load("images/rooms/office/talents_on_desk.png");

    elements_on_desk.spawn(ImageBundle {
        image: mission_report_documents_image.into(),
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            top: Val::Px(50.),
            right: Val::Px(50.),
            width: Val::Px(600. / 4.),
            height: Val::Px(800. / 4.),
            ..default()
        },
        ..default()
    });
}
