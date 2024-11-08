use bevy::{
    prelude::*,
    ui::{AlignItems, JustifyContent, Val},
};

#[allow(dead_code)]
pub fn inventory_filter_button_style() -> Node {
    Node {
        width: Val::Percent(100. / 4.),
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        border: UiRect::all(Val::Px(3.)),
        height: Val::Px(40.),
        ..default()
    }
}
