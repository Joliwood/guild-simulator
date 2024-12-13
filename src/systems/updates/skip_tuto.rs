use crate::{
    content::equipments::weapons::WeaponsEnum,
    structs::{
        equipments::ItemEnum,
        general_structs::TutoMessagesModalVisible,
        player_stats::{PlayerStats, TutoMessages},
        trigger_structs::SkipTutoMessageTrigger,
    },
};
use bevy::prelude::*;

pub fn skip_tuto(
    mut query: Query<&Interaction, (Changed<Interaction>, With<SkipTutoMessageTrigger>)>,
    mut tuto_message_modal_visibility: ResMut<TutoMessagesModalVisible>,
    mut tuto_messages: ResMut<TutoMessages>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                tuto_message_modal_visibility.0 = false;
                tuto_messages.0.clear();
                player_stats.tuto.reset();
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
