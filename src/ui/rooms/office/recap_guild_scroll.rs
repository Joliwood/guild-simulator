use bevy::prelude::*;

pub fn recap_guild_scroll(asset_server: &Res<AssetServer>, elements_on_desk: &mut ChildBuilder) {
    let mission_report_documents_image: Handle<Image> =
        asset_server.load("images/rooms/office/recap_guild_scroll.png");

    elements_on_desk.spawn(ImageBundle {
        image: mission_report_documents_image.into(),
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            width: Val::Px(550. / 2.),
            height: Val::Px(140. / 2.),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
