use crate::{
    enums::RoomEnum,
    structs::{player_stats::PlayerStats, trigger_structs::RoomButtonTrigger},
};
use bevy::prelude::*;

// System to update room based on button clicks
pub fn update_room_on_click(
    mut interaction_query: Query<
        (&Interaction, &RoomButtonTrigger, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    for (interaction, room_button, mut _background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                // background_color.0 = Color::srgba(0., 0., 0., 0.6);
            }
            Interaction::Pressed => match room_button.0 {
                RoomEnum::CommandRoom => player_stats.room = RoomEnum::CommandRoom,
                RoomEnum::Office => player_stats.room = RoomEnum::Office,
                RoomEnum::Barrack => player_stats.room = RoomEnum::Barrack,
                _ => (),
            },
            Interaction::None => {
                // background_color.0 = Color::NONE;
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
