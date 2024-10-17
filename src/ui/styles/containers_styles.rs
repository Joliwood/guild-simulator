use bevy::{
    prelude::*,
    ui::{AlignItems, JustifyContent, PositionType, Style, Val},
};

pub fn node_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

pub fn room_interface_text_style() -> Style {
    Style {
        // The position absolute make the gold counter visible (z-index)
        position_type: PositionType::Absolute,
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        display: Display::Flex,
        padding: UiRect::all(Val::Px(10.0)),
        row_gap: Val::Px(10.0),
        align_items: AlignItems::Center,
        width: Val::Auto,
        height: Val::Px(36.0),
        ..default()
    }
}

pub fn room_arrow_button_style() -> Style {
    Style {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        margin: UiRect::all(Val::Auto),
        align_items: AlignItems::Center,
        width: Val::Px(36.0),
        height: Val::Px(36.0),
        ..default()
    }
}

pub fn basic_button_style() -> Style {
    Style {
        display: Display::Flex,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        width: Val::Auto,
        height: Val::Px(36.0),
        ..default()
    }
}
