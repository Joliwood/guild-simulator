use bevy::{
  // ecs::{query::With, system::{Query, Res}},
  ecs::system::Res, input::{mouse::MouseButton, ButtonInput}, log::info
  // window::{PrimaryWindow, Window},
};

// This system prints messages when you press or release the left mouse button:
pub fn mouse_click_system(
  mouse_button_input: Res<ButtonInput<MouseButton>>,
  // q_windows: Query<&Window, With<PrimaryWindow>>
) {
  // Left Mouse Button
  if mouse_button_input.pressed(MouseButton::Left) {
      // info!("Left mouse button currently pressed.");
      // info!("{:?}", q_windows);
      // if let Some(position) = q_windows.single().cursor_position() {
      //     println!("Cursor is inside the primary window, at {:?}", position);
      // } else {
      //     println!("Cursor is not in the game window.");
      // }
  }
  // if mouse_button_input.just_pressed(MouseButton::Left) {
  //     info!("Left mouse button just pressed.");
  // }
  if mouse_button_input.just_released(MouseButton::Left) {
      info!("Left mouse button just released.");
  }

  // Right Mouse Button
  // if mouse_button_input.pressed(MouseButton::Right) {
  //     info!("Right mouse button currently pressed.");
  // }
  // if mouse_button_input.just_pressed(MouseButton::Right) {
  //     info!("Right mouse button just pressed.");
  // }
  if mouse_button_input.just_released(MouseButton::Right) {
      info!("Right mouse button just released.");
  }
}
