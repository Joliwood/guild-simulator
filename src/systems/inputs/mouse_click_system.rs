use bevy::{
    // ecs::{query::With, system::{Query, Res}},
    ecs::system::Res,
    input::{mouse::MouseButton, ButtonInput},
    log::info, // window::{PrimaryWindow, Window},
};

// This system prints messages when you press or release the left mouse button:
pub fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        // window.set_cursor_icon(CursorIcon::Hand);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        // info!("Left mouse button just released.");
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        info!("Right mouse button just released.");
    }
}
