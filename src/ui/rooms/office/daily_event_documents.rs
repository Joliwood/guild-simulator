use crate::my_assets::MyAssets;
use crate::structs::daily_events_folder::daily_events::DailyEvents;
use crate::structs::trigger_structs::DailyEventTrigger;
use bevy::prelude::*;

pub fn daily_event_documents(
    my_assets: &Res<MyAssets>,
    elements_on_desk: &mut ChildBuilder,
    daily_events: &Res<DailyEvents>,
) {
    let daily_events_number = daily_events.0.len();

    if daily_events_number > 0 {
        elements_on_desk
            .spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(50.),
                    top: Val::Px(250.),
                    ..default()
                },
            ))
            // .insert(MissionReportButtonTrigger)
            .insert(DailyEventTrigger)
            .with_children(|mission_report_button| {
                mission_report_button
                    .spawn((
                        UiImage {
                            image: my_assets.daily_event_documents_on_desk.clone().into(),
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
                                overlay.spawn((
                                    Text::new(format!("{}", daily_events_number)),
                                    TextFont {
                                        font: my_assets.fira_sans_bold.clone(),
                                        font_size: 25.0,
                                        ..default()
                                    },
                                    TextColor(Color::BLACK),
                                ));
                            });
                    });
            });
    }
}
