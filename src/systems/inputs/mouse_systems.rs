use bevy::{
    ecs::system::Res,
    input::{mouse::MouseButton, ButtonInput},
    log::info,
    prelude::*,
    window::CursorGrabMode,
};

/// This system prints messages when you press or release the left mouse button:
pub fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Grabbing;
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Pointer;
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        info!("Right mouse button just released.");
    }
}

pub fn mouse_init(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;
}
