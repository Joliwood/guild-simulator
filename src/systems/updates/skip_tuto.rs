use crate::structs::{general_structs::MissionModalVisible, trigger_structs::SkipTutoTrigger};
use bevy::prelude::*;

pub fn skip_tuto(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<SkipTutoTrigger>),
    >,
    mut modal_visible: ResMut<MissionModalVisible>,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                modal_visible.0 = false;
                border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;@#<><>>>>
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
                border_color.0 = Color::BLACK;
            }
        }
    }
}
