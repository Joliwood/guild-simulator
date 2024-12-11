use crate::{
    custom_components::get_mission_position,
    my_assets::{get_mission_image, FONT_FIRA},
    structs::{general_structs::UniqueId, missions::Mission},
};
use bevy::prelude::*;

/// The boolean is if a recruit is available or not
#[derive(Component)]
pub struct MissionIconTrigger;

#[derive(Component)]
pub struct MissionIconUnavailableTrigger(pub u16);

pub fn mission_icon(my_assets: &Res<AssetServer>, parent: &mut ChildBuilder, mission: &Mission) {
    let mission_position = get_mission_position(mission.id);
    let mission_image_path = get_mission_image(mission.id);

    // if mission.recruit_send.is_none() {
    parent
        .spawn((
            Button,
            ImageNode {
                image: my_assets.load("images/missions/mission_icon.png"),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(130.),
                height: Val::Px(130.),
                left: Val::Px(mission_position.0),
                top: Val::Px(mission_position.1),
                ..default()
            },
            UniqueId("select_mission_button".to_string()),
            mission.clone(),
            GlobalZIndex(2),
            MissionIconTrigger,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: my_assets.load(&mission_image_path),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(90.),
                    height: Val::Px(90.),
                    top: Val::Px(15.),
                    ..default()
                },
                BorderRadius::MAX,
                GlobalZIndex(1),
            ));

            parent
                .spawn((
                    ImageNode {
                        image: my_assets.load(mission_image_path),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        display: Display::None,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(90.),
                        height: Val::Px(90.),
                        top: Val::Px(15.),
                        ..default()
                    },
                    BorderRadius::MAX,
                    GlobalZIndex(1),
                    MissionIconUnavailableTrigger(mission.id),
                ))
                .with_children(|overlay| {
                    overlay.spawn((
                        Text::new(t!("recruit_already_sent")),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 11.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
