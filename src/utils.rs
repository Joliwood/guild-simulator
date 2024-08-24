use crate::{
    enums::{RoomDirectionEnum, RoomEnum},
    structs::general_structs::PlayerStats,
};
use bevy::prelude::ResMut;

/// Determines the new room based on the given direction and current player stats.
///
/// # Parameters
/// - `player_stats`: The current player stats containing the current room.
/// - `direction`: The direction in which the room change is requested.
///
/// # Returns
/// The new room enum corresponding to the direction.
pub fn get_new_room(
    player_stats: &ResMut<PlayerStats>,
    direction: RoomDirectionEnum,
) -> Option<RoomEnum> {
    match player_stats.room {
        RoomEnum::Office => match direction {
            RoomDirectionEnum::Right => Some(RoomEnum::Barrack),
            RoomDirectionEnum::Left => Some(RoomEnum::Store),
            RoomDirectionEnum::Bottom => Some(RoomEnum::CommandRoom),
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::Barrack => match direction {
            RoomDirectionEnum::Right => None,
            RoomDirectionEnum::Left => Some(RoomEnum::Office),
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::Store => match direction {
            RoomDirectionEnum::Right => Some(RoomEnum::Office),
            RoomDirectionEnum::Left => None,
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => None,
        },
        RoomEnum::CommandRoom => match direction {
            RoomDirectionEnum::Right => None,
            RoomDirectionEnum::Left => None,
            RoomDirectionEnum::Bottom => None,
            RoomDirectionEnum::Top => Some(RoomEnum::Office),
        },
    }
}

// pub fn increment_golds(player_stats: &mut ResMut<PlayerStats>, amount: i32) {
//     player_stats.golds += amount;
// }

// pub fn select_recruit(mut selected_recruit: ResMut<SelectedRecruit>, recruit: RecruitStats) {
//     selected_recruit.0 = Some(recruit);
// }

/// Calculates the total points of a recruit based on its strength, endurance
/// and intelligence.
///
/// Basic V1 algorythme to calculate after that the % of victory for each
/// mission and each recruit assigned
pub fn get_global_points(strength: u16, endurance: u16, intelligence: u16) -> u16 {
    return strength + endurance + intelligence;
}

/// Calculates the victory percentage of a mission based on the global points
///
/// ## Description
/// - Si la recrue a 2 fois moins de points que l'ennemi, il a 0% de chance de gagner
/// - Si la recrue a 2 fois plus de points que l'ennemi, il a 100% de chance de gagner
/// - Si la recrue a autant de points que l'ennemi, il a 50% de chance de gagner
///
/// ## Returns
/// The victory percentage of the mission.
pub fn get_victory_percentage(recruit_global_points: u16, enemy_global_points: u16) -> f32 {
    let loose_guaranteed: u16 = enemy_global_points / 2;
    let victory_guaranteed: u16 = enemy_global_points * 2;
    let percent_per_point_lower_range: f32 = 50.0 / loose_guaranteed as f32;
    let percent_per_point_upper_range: f32 = 50.0 / enemy_global_points as f32;

    if recruit_global_points <= loose_guaranteed {
        return 0.;
    } else if recruit_global_points > loose_guaranteed
        && recruit_global_points < enemy_global_points
    {
        return (recruit_global_points - loose_guaranteed) as f32 * percent_per_point_lower_range;
    } else if recruit_global_points == enemy_global_points {
        return 50.;
    } else if recruit_global_points > enemy_global_points
        && recruit_global_points < victory_guaranteed
    {
        return 100.
            - (victory_guaranteed - recruit_global_points) as f32 * percent_per_point_upper_range;
    } else {
        // recruit_global_points >= enemy_global_points * 2
        return 100.;
    }
}

/// Use the % of victory to randomly determine if the mission is a success or a failure
///
/// ## Parameters
/// - `percent_of_victory`: The % of victory of the mission (0 to 100)
///
/// It will generate a random number between 0 and 1 and compare it to the % of victory divided by 100
pub fn is_mission_success(percent_of_victory: f32) -> bool {
    let random_number: f32 = rand::random();
    return random_number <= percent_of_victory / 100.;
}

/// Get the xp earned on a mission based on the level of the ennemy
///
/// This function will can be upgraded to the recruit level, here some futur examples :
/// - recruit level < 5 ennemy level -> / 10 xp
/// - recruit level > 3 ennemy level -> x3 xp
pub fn get_xp_earned(level: u8) -> u32 {
    return (level * 10).into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_global_points() {
        assert_eq!(get_global_points(1, 1, 1), 3);
        assert_eq!(get_global_points(2, 2, 2), 6);
        assert_eq!(get_global_points(3, 3, 3), 9);
    }

    #[test]
    fn test_get_victory_percentage() {
        assert_eq!(get_victory_percentage(5, 20), 0.);
        assert_eq!(get_victory_percentage(10, 20), 0.);
        assert_eq!(get_victory_percentage(20, 20), 50.);
        assert_eq!(get_victory_percentage(25, 20), 62.5);
        assert_eq!(get_victory_percentage(40, 20), 100.);
        assert_eq!(get_victory_percentage(50, 20), 100.);
    }
}
