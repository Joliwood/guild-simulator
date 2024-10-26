use crate::{
    my_assets::MyAssets,
    structs::{missions::MissionReports, trigger_structs::SleepButtonTrigger},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn set_of_keys(
    my_assets: &Res<MyAssets>,
    elements_on_desk: &mut ChildBuilder,
    mission_reports: &Res<MissionReports>,
) {
    let tooltip_text =  "You have to sign all the\nreports before going to sleep.\n\nNo pain no gains !\nAs the kids say";

    // Create the main button
    let mut button = elements_on_desk.spawn(ButtonBundle {
        style: Style {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            top: Val::Px(50.),
            right: Val::Px(50.),
            width: Val::Px(100.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Px(3.)),
            ..default()
        },
        border_color: BorderColor(Color::NONE),
        border_radius: BorderRadius::all(Val::Percent(100.)),
        ..default()
    });

    // Insert the tooltip only if there are mission reports
    if !mission_reports.0.is_empty() {
        button.insert(
            Tooltip::cursor(tooltip_text.to_string()).with_activation(TooltipActivation::IDLE),
        );
    }

    // Add the trigger and children
    button
        .insert(SleepButtonTrigger)
        .with_children(|sleep_button| {
            sleep_button.spawn(ImageBundle {
                image: my_assets.set_of_keys_container.clone().into(),
                style: Style {
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            });
        });

    // Spawn the image inside the button as a child
    button.with_children(|parent| {
        parent.spawn(ImageBundle {
            image: my_assets.set_of_keys.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(15.),
                    bottom: Val::Auto,
                },
                width: Val::Percent(65.),
                height: Val::Percent(65.),
                ..default()
            },
            ..default()
        });
    });
}
