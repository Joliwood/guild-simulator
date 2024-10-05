use crate::{structs::trigger_structs::MissionReport, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

pub fn mission_report_documents(my_assets: &Res<MyAssets>, elements_on_desk: &mut ChildBuilder) {
    elements_on_desk
        .spawn(ImageBundle {
            image: my_assets.mission_notification_document.clone().into(),
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
