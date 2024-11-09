#![allow(dead_code)]
use crate::{my_assets::get_mission_image, ui::ui_constants::WOOD_COLOR};
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
    pub fn bundle(
        &self,
        my_assets: &Res<AssetServer>,
    ) -> (
        Button,
        Node,
        BorderRadius,
        BackgroundColor,
        UiImage,
        BorderColor,
    ) {
        // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px

        match self {
            CustomButton::Primary => (
                Button,
                Node {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor(WOOD_COLOR),
                UiImage::default(),
                BorderColor(Color::BLACK),
            ),
            CustomButton::GoldButton => (
                Button,
                Node {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(5.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor::DEFAULT,
                UiImage::default(),
                BorderColor::DEFAULT,
            ),
            // image_assets
            CustomButton::RoomArrow => (
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor::DEFAULT,
                UiImage::default().with_color(Color::WHITE),
                BorderColor(Color::BLACK),
            ),
            CustomButton::SquareIcon => (
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(50.),
                    aspect_ratio: Some(1.),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor(WOOD_COLOR),
                UiImage::default().with_color(Color::WHITE),
                BorderColor(Color::BLACK),
            ),
            CustomButton::EarnGold => (
                Button,
                Node {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                // my_assets.buttons_atlas.clone().into(),
                my_assets.load("images/hud/buttons_atlas.png").into(),
                BorderColor(WOOD_COLOR),
            ),
            CustomButton::MissionStart => (
                Button,
                Node {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Px(200.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                UiImage {
                    image: my_assets.load("images/rooms/command_room/wood_box_container.png"),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                BorderColor(Color::BLACK),
            ),
            _ => panic!("The mission button has to be called in the mission_bundle method"),
        }
    }

    pub fn mission_bundle(
        &self,
        my_assets: &Res<AssetServer>,
        mission_id: u16,
    ) -> (
        Button,
        Node,
        BorderRadius,
        BackgroundColor,
        UiImage,
        BorderColor,
    ) {
        let mission_image = get_mission_image(mission_id);
        let mission_position = get_mission_position(mission_id);

        match self {
            CustomButton::MissionOnMap => (
                Button,
                Node {
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
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                my_assets.load(mission_image).into(),
                BorderColor(Color::BLACK),
            ),
            _ => panic!("This button is not a mission on map button"),
        }
    }
}

pub fn get_mission_position(mission_id: u16) -> (f32, f32) {
    match mission_id {
        1 => (370., 270.),
        2 => (200., 270.),
        3 => (35., 270.),
        4 => (370., 60.),
        5 => (200., 60.),
        6 => (35., 60.),
        _ => panic!("Mission id not found"),
    }
}
