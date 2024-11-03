use crate::my_assets::MyAssets;
use crate::structs::missions::MissionReports;
use crate::structs::trigger_structs::MissionReportTrigger;
// use crate::structs::trigger_structs::MissionReportButtonTrigger;
use bevy::prelude::*;

pub fn mission_report_documents(
    my_assets: &Res<MyAssets>,
    elements_on_desk: &mut ChildBuilder,
    mission_reports: &Res<MissionReports>,
) {
    let mission_reports_number = mission_reports.0.len();

    if mission_reports_number > 0 {
        elements_on_desk
            .spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(50.),
                    top: Val::Px(50.),
                    ..default()
                },
            ))
            // .insert(MissionReportButtonTrigger)
            .insert(MissionReportTrigger)
            .with_children(|mission_report_button| {
                mission_report_button
                    .spawn((
                        UiImage {
                            image: my_assets.mission_notification_document.clone().into(),
                            ..default()
                        },
                        Node {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(150.),
                            height: Val::Px(133. + 66.5),
                            ..default()
                        },
                    ))
                    // .insert(MissionReport)
                    .insert(Interaction::default())
                    .with_children(|parent| {
                        // if mission_reports_number > 0 {
                        parent
                            .spawn((
                                UiImage {
                                    image: my_assets.notification_token_in_wood.clone().into(),
                                    ..default()
                                },
                                Node {
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Px(40.),
                                    height: Val::Px(40.),
                                    right: Val::Px(5.),
                                    top: Val::Px(5.),
                                    ..default()
                                },
                            ))
                            .with_children(|overlay| {
                                overlay.spawn(TextBundle::from_section(
                                    format!("{}", mission_reports_number),
                                    TextFont {
                                        font: my_assets.fira_sans_bold.clone(),
                                        font_size: 25.0,
                                        color: Color::BLACK,
                                    },
                                ));
                            });
                    });
            });
    }
}
