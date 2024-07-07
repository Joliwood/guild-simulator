use bevy::{
    prelude::default,
    ui::{AlignItems, JustifyContent, Style, Val},
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
