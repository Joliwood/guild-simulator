use crate::structs::{SelectedRecruit, SelectedRecruitTrigger};
use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_recruit_infos(
    selected_recruit: Res<SelectedRecruit>,
    mut query: Query<&mut Text, With<SelectedRecruitTrigger>>,
) -> () {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:?}", selected_recruit);
    }
}
