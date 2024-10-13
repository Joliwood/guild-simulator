use crate::{structs::trigger_structs::SleepButtonTrigger, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

pub fn set_of_keys(my_assets: &Res<MyAssets>, elements_on_desk: &mut ChildBuilder) {
    elements_on_desk
        .spawn(ButtonBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                // Futur position when talents will be disponible
                // top: Val::Px(280.),
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
        })
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
        })
        .with_children(|parent| {
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
