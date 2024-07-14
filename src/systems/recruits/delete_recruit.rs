use crate::structs::{PlayerStats, RecruitStats};
use bevy::prelude::*;
use uuid::Uuid;

/// Checks for changes in PlayerStats and updates the room accordingly
///
/// # Parameters
/// - `player_stats`: We will add, delete, or update one recruit
/// - `recruit_uuid`: The recruit to target
/// - `recruit_infos`: The new stats of the recruit (only what is updated)
///
/// # Returns
/// - `bool`: True if the recruit has been updated
/// - `bool`: False if we couldn't update the recruit (for exemple, equip a shield on a mage)
pub fn update_one_recruit(
    mut player_stats: ResMut<PlayerStats>,
    recruit_id: Uuid,
    recruit_infos: Option<RecruitStats>,
) -> bool {
    // 1 - We find the recruit with the uui in the player_stats.recruits
    // 2 - We update the recruit with the new stats
    // 3 - We return true if the recruit update is success

    // TODO

    return true;
}
