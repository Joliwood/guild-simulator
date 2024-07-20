use crate::{systems::systems_constants::NORMAL_BUTTON, ui::ui_constants::WOOD_COLOR};
use bevy::prelude::*;

pub enum CustomButton {
    Primary,
    GoldButton,
    RoomArrow,
}

impl CustomButton {
    pub fn bundle(&self, asset_server: &Res<AssetServer>) -> ButtonBundle {
        match self {
            CustomButton::Primary => ButtonBundle {
                style: Style {
                    border: UiRect::all(Val::Px(5.0)),
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
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
                    margin: UiRect::all(Val::Px(10.0)),
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::MAX,
                // Test for insert image asset for button
                // image: UiImage::new(asset_server.load("../assets/images/cursors/cursor_grab.png")),
                ..Default::default()
            },
            CustomButton::RoomArrow => ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                image: UiImage::default().with_color(NORMAL_BUTTON),
                ..default()
            },
        }
    }
}
