use crate::{structs::trigger_structs::MissionReport, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

use crate::structs::{missions::MissionReports, trigger_structs::MissionReport};

pub fn mission_report_documents(
    asset_server: &Res<AssetServer>,
    elements_on_desk: &mut ChildBuilder,
    mission_reports: ResMut<MissionReports>,
) {
    let mission_report_documents_image: Handle<Image> =
        asset_server.load("images/rooms/office/mission_notification_document.png");

    let token_image: Handle<Image> =
        asset_server.load("images/rooms/office/notification_token_in_wood.png");

    let mission_reports_number = mission_reports.0.len();

    elements_on_desk
        .spawn(ImageBundle {
            image: my_assets.mission_notification_document.clone().into(),
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
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
        .insert(Interaction::default())
        .with_children(|parent| {
            if mission_reports_number > 0 {
                parent
                    .spawn(ImageBundle {
                        image: token_image.into(),
                        style: Style {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(40.),
                            height: Val::Px(40.),
                            right: Val::Px(5.),
                            top: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|overlay| {
                        overlay.spawn(TextBundle::from_section(
                            format!("{}", mission_reports_number),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::BLACK,
                            },
                        ));
                    });
            }
        });
}
