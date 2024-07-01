use crate::structs::PlayerStats;
use crate::systems::constants::*;
use bevy::prelude::*;

pub fn menu_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut player_stats: ResMut<PlayerStats>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        // Safely get the child text component
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    println!("Button pressed");
                    text.sections[0].value = "O".to_string();
                    player_stats.increment_golds(1);
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::srgba(255.0, 0.0, 0.0, 255.0);
                }
                Interaction::Hovered => {
                    text.sections[0].value = "H".to_string();
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                    window.cursor.icon = CursorIcon::Grab;
                }
                Interaction::None => {
                    text.sections[0].value = "X".to_string();
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                    window.cursor.icon = CursorIcon::Pointer;
                }
            }
        }
    }
}
