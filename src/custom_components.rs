#![allow(dead_code)]
use crate::{
    systems::systems_constants::NORMAL_BUTTON,
    ui::{interface::gold_counter::MyAssets, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

pub enum CustomButton {
    Primary,
    GoldButton,
    RoomArrow,
    SquareIcon,
    EarnGold,
    MissionStart,
    MissionOnMap,
}

impl CustomButton {
    pub fn bundle(&self, my_assets: &Res<MyAssets>) -> ButtonBundle {
        // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px

        match self {
            CustomButton::Primary => ButtonBundle {
                style: Style {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::MAX,
                background_color: BackgroundColor(WOOD_COLOR),
                ..Default::default()
            },
            CustomButton::GoldButton => ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(5.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::MAX,
                // Test for insert image asset for button
                // image: UiImage::new(my_assets.load("../assets/images/cursors/cursor_grab.png")),
                // image: UiImage::new(image_assets.test_button),
                ..Default::default()
            },
            // image_assets
            CustomButton::RoomArrow => ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                image: UiImage::default().with_color(NORMAL_BUTTON),
                ..default()
            },
            CustomButton::SquareIcon => ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(50.),
                    aspect_ratio: Some(1.),
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(10.)),
                background_color: BackgroundColor(WOOD_COLOR),
                border_color: BorderColor(Color::BLACK),
                image: UiImage::default().with_color(NORMAL_BUTTON),
                ..default()
            },
            CustomButton::EarnGold => ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                image: my_assets.buttons_atlas.clone().into(),
                border_color: BorderColor(WOOD_COLOR),
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
            CustomButton::MissionStart => ButtonBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Px(200.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                image: my_assets.wood_box_container.clone().into(),
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
            _ => panic!("The mission button has to be called in the mission_bundle method"),
        }
    }

    pub fn mission_bundle(&self, my_assets: &Res<MyAssets>, mission_id: u16) -> ButtonBundle {
        let mission_image = my_assets.get_mission_image(mission_id);
        let mission_position = get_mission_position(mission_id);

        match self {
            CustomButton::MissionOnMap => ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(mission_position.0),
                    top: Val::Px(mission_position.1),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Px(130.),
                    height: Val::Px(130.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                image: mission_image.clone().into(),
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
            _ => panic!("This button is not a mission on map button"),
        }
    }
}

pub fn get_mission_position(mission_id: u16) -> (f32, f32) {
    match mission_id {
        1 => (400., 270.),
        2 => (220., 270.),
        3 => (35., 270.),
        4 => (400., 60.),
        5 => (220., 60.),
        6 => (35., 60.),
        _ => panic!("Mission id not found"),
    }
}
