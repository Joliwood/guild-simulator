use crate::{
    enums::RecruitStateEnum,
    structs::{
        general_structs::UniqueId,
        recruits::{RecruitStats, SelectedRecruit},
    },
    systems::systems_constants::HOVERED_BUTTON,
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

/// Select the recruit when the button is pressed
pub fn select_recruit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit: ResMut<SelectedRecruit>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, unique_id, recruit) in &mut interaction_query {
        let recruit_state = recruit.clone().state;
        if unique_id.0 == "recruit_button" && recruit_state != RecruitStateEnum::InMission {
            match *interaction {
                Interaction::Pressed => {
                    selected_recruit.0 = Some(recruit.clone());
                }
                Interaction::Hovered => {
                    window.cursor.icon = CursorIcon::Pointer;
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    window.cursor.icon = CursorIcon::Default;
                    *color = BackgroundColor(WOOD_COLOR);
                }
            }
        }
    }
}
