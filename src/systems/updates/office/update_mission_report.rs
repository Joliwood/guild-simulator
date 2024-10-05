use crate::{structs::trigger_structs::MissionReport, systems::systems_constants::HOVERED_BUTTON};
use bevy::prelude::*;

pub fn update_mission_report(
    mut query: Query<
        (&Interaction, &mut Style, &mut BackgroundColor),
        (Changed<Interaction>, With<MissionReport>),
    >,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut style, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                *color = HOVERED_BUTTON.into();
                // Add a border when hovered
                style.border = UiRect {
                    left: Val::Px(3.0),
                    right: Val::Px(3.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(3.0),
                };
                BorderColor(Color::WHITE);
            }
            Interaction::None => {
                // Remove the border when not interacted with
                window.cursor.icon = CursorIcon::Default;
                *color = Color::NONE.into();
                style.border = UiRect::default();
            }
            _ => {}
        }
    }
}
