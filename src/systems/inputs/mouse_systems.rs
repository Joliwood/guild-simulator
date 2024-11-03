use bevy::{
    ecs::system::Res,
    input::{mouse::MouseButton, ButtonInput},
    prelude::*,
    window::CursorGrabMode,
    // window::CursorGrabMode,
};

/// This system log messages when you press or release the left mouse button:
pub fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) {
        // window.cursor.icon = CursorIcon::Grabbing;
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        // window.cursor.icon = CursorIcon::Default;
    }

    if mouse_button_input.just_released(MouseButton::Right) {}
}

// pub fn mouse_init(mut windows: Query<&mut Window>) {
//     let mut window = windows.single_mut();
//     // window.cursor.icon = CursorIcon::Default;
// }
