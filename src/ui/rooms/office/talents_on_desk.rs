use crate::ui::interface::gold_counter::MyAssets;
use bevy::prelude::*;

pub fn talents_on_desk(my_assets: &Res<AssetServer>, elements_on_desk: &mut ChildBuilder) {
    elements_on_desk.spawn(ImageBundle {
        image: my_assets.talents_on_desk.clone().into(),
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
