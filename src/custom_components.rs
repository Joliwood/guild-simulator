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
}

impl CustomButton {
    pub fn bundle(
        &self,
        _asset_server: &Res<AssetServer>,
        image_assets: &Res<MyAssets>,
    ) -> ButtonBundle {
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
                // image: UiImage::new(asset_server.load("../assets/images/cursors/cursor_grab.png")),
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
                image: image_assets.test_button.clone().into(),
                border_color: BorderColor(WOOD_COLOR),
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
        }
    }
}
