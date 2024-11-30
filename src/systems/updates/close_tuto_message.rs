use crate::structs::{
    general_structs::TutoMessagesModalVisible, trigger_structs::CloseTutoMessageTrigger,
};
use bevy::prelude::*;

pub fn close_tuto_message(
    _my_assets: Res<AssetServer>,
    _commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<CloseTutoMessageTrigger>)>,
    mut tuto_message_modal_visibility: ResMut<TutoMessagesModalVisible>,
) {
    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                tuto_message_modal_visibility.0 = false;
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
