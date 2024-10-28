use crate::{
    my_assets::MyAssets,
    structs::daily_events_folder::spontaneous_applications::SpontaneousApplication,
};
use bevy::prelude::*;

pub fn spontaneous_application_event_doc(
    commands: &mut Commands,
    my_assets: &Res<MyAssets>,
    spontaneous_application: SpontaneousApplication,
) {
    commands
        .spawn(ImageBundle {
            image: my_assets.daily_event_document.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                width: Val::Px(330.),
                height: Val::Px(500.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    &spontaneous_application.title,
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });
}
