use crate::{
    enums::{RecruitEnum, RoomDirectionEnum, RoomEnum},
    structs::{
        equipments::Item,
        general_structs::{PlayerStats, RecruitInventory, RecruitStats, SelectedRecruit},
    },
    ui::ui_constants::{ARMOR_PATH, SCROLL_PATH, WEAPON_PATH},
};
use bevy::{
    math::UVec2,
    prelude::{Res, ResMut},
    sprite::TextureAtlasLayout,
};
use uuid::Uuid;

/// Determines the new room based on the given direction and current player stats.
///
/// ## Parameters
/// - `player_stats`: The current player stats containing the current room.
/// - `direction`: The direction in which the room change is requested.
///
/// ## Returns
/// The new room enum corresponding to the direction.
///
/// ## Permanently the store room has been removed for V0
pub fn get_new_room(
    player_stats: &ResMut<PlayerStats>,
    direction: RoomDirectionEnum,
) -> Option<RoomEnum> {
    match player_stats.room {
        RoomEnum::Office => match direction {
            RoomDirectionEnum::Right => Some(RoomEnum::Barrack),
            // RoomDirectionEnum::Left => Some(RoomEnum::Store),
            RoomDirectionEnum::Left => None,
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

/// Calculates the total points of a recruit based on its strength, endurance
/// and intelligence.
///
/// Basic V1 algorythme to calculate after that the % of victory for each
/// mission and each recruit assigned
pub fn get_global_points(strength: u16, endurance: u16, intelligence: u16) -> u16 {
    return strength + endurance + intelligence;
}

/// ## Calculates the victory percentage of a mission based on the global points
///
/// - If recruit points = ennemy points / 2 => 0%.
/// - If recruit points = ennemy points x 2 => 100%.
/// - If recruits points  = ennemy points => 50%.
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

pub fn format_ron_equipments_for_display(ron_data: &str) -> String {
    // Use a regex to format the RON output
    let formatted = ron_data
        .replace(":", ": ")
        .replace("),", "},")
        .replace(")", "}")
        .replace("Armors(", "  {")
        .replace("Armors(", "{")
        .replace("items:", "\"items\":")
        .replace("Weapon(", "  {")
        .replace("Weapons(", "  {")
        .replace("Weapons(", "{")
        .replace("Weapons(", "{");

    formatted
}

/// Get the image atlas index of an item
///
/// Has to be updated each time the design will evolve
pub fn get_item_image_atlas_index(item: &Item) -> u16 {
    return match item {
        Item::Weapon(weapon) => weapon.image_atlas_index,
        Item::Armor(armor) => armor.image_atlas_index,
        Item::Scroll(scroll, _) => scroll.image_atlas_index,
    };
}

/// Get the path of the image atlas of an item
///
/// Has to be updated each time the design will evolve
pub fn get_item_atlas_path(item: &Item) -> String {
    return match item {
        Item::Weapon(_) => WEAPON_PATH.to_string(),
        Item::Armor(_) => ARMOR_PATH.to_string(),
        Item::Scroll(_, _) => SCROLL_PATH.to_string(),
    };
}

/// Get the layout of the image atlas of an item
///
/// Has to be updated each time the design will evolve
pub fn get_item_layout(item: &Item) -> TextureAtlasLayout {
    return match item {
        Item::Weapon(_) => TextureAtlasLayout::from_grid(
            UVec2::new(2900, 400),
            6,
            1,
            Some(UVec2::new(0, 0)),
            Some(UVec2::new(0, 0)),
        ),
        Item::Armor(_) => TextureAtlasLayout::from_grid(
            UVec2::new(1600, 400),
            4,
            1,
            Some(UVec2::new(0, 0)),
            Some(UVec2::new(0, 0)),
        ),
        Item::Scroll(_, _) => TextureAtlasLayout::from_grid(
            UVec2::new(4320, 1080),
            4,
            1,
            Some(UVec2::new(0, 0)),
            Some(UVec2::new(0, 0)),
        ),
    };
}

/// Get the tooltip description of an item
///
/// For now, only supports texts
pub fn get_item_tooltip_description(item: &Item) -> String {
    return match item {
        Item::Weapon(weapon) => {
            let mut description = format!("{}\n", weapon.name);
            let price_range = calculate_price_range(weapon.price);

            if let Some(endurance) = weapon.endurance {
                description.push_str(&format!("\nEndurance: {}", endurance));
            }
            if let Some(strength) = weapon.strength {
                description.push_str(&format!("\nStrength: {}", strength));
            }
            if let Some(intelligence) = weapon.intelligence {
                description.push_str(&format!("\nIntelligence: {}", intelligence));
            }
            description.push_str(&format!(
                "\n\nPrice: {} to {} G",
                price_range.0, price_range.1
            ));

            description
        }
        Item::Armor(armor) => {
            let mut description = format!("{}\n", armor.name);
            let price_range = calculate_price_range(armor.price);

            if let Some(endurance) = armor.endurance {
                description.push_str(&format!("\nEndurance: {}", endurance));
            }
            if let Some(strength) = armor.strength {
                description.push_str(&format!("\nStrength: {}", strength));
            }
            if let Some(intelligence) = armor.intelligence {
                description.push_str(&format!("\nIntelligence: {}", intelligence));
            }
            description.push_str(&format!(
                "\n\nPrice: {} to {} G",
                price_range.0, price_range.1
            ));

            description
        }
        Item::Scroll(scroll, quantity) => {
            let mut description = format!("{}\n", scroll.name);

            if let Some(endurance) = scroll.endurance {
                description.push_str(&format!("\nEndurance: {}", endurance));
            }
            if let Some(strength) = scroll.strength {
                description.push_str(&format!("\nStrength: {}", strength));
            }
            if let Some(intelligence) = scroll.intelligence {
                description.push_str(&format!("\nIntelligence: {}", intelligence));
            }
            description.push_str(&format!("\nQuantity: {}", quantity));
            description.push_str(&format!("\n\nPrice: {} G", scroll.price));

            description
        }
    };
}

/// Calculate the price range of an item based on its price
///
/// The item will can be sold more or less depending on merchant affinities
pub fn calculate_price_range(price: u16) -> (u16, u16) {
    let lower_range = (price as f32 * 0.95) as u16;
    let upper_range = (price as f32 * 1.05) as u16;

    (lower_range, upper_range)
}

pub fn get_mission_notification_tooltip_text(completed_mission_number: u8) -> String {
    let mission_word = if completed_mission_number > 1 {
        "missions"
    } else {
        "mission"
    };

    format!(
        "You have completed {} {}.

You should check out your mission
reports in your office.

Click to dismiss.",
        completed_mission_number, mission_word
    )
}

pub fn get_selected_recruit(selected_recruit: &Res<SelectedRecruit>) -> RecruitStats {
    match selected_recruit.0 {
        Some(_) => {
            return RecruitStats {
                recruit_inventory: selected_recruit
                    .0
                    .as_ref()
                    .unwrap()
                    .recruit_inventory
                    .clone(),
                class: selected_recruit.0.as_ref().unwrap().class.clone(),
                endurance: selected_recruit.0.as_ref().unwrap().endurance,
                experience: selected_recruit.0.as_ref().unwrap().experience,
                id: selected_recruit.0.as_ref().unwrap().id,
                image_atlas_index: selected_recruit.0.as_ref().unwrap().image_atlas_index,
                intelligence: selected_recruit.0.as_ref().unwrap().intelligence,
                level: selected_recruit.0.as_ref().unwrap().level,
                max_experience: selected_recruit.0.as_ref().unwrap().max_experience,
                name: selected_recruit.0.as_ref().unwrap().name.clone(),
                strength: selected_recruit.0.as_ref().unwrap().strength,
            };
        }
        None => {
            return RecruitStats {
                recruit_inventory: RecruitInventory::generate_empty_inventory(),
                class: RecruitEnum::Warrior,
                endurance: 0,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 4,
                intelligence: 0,
                level: 0,
                max_experience: 0,
                name: "".to_string(),
                strength: 0,
            };
        }
    }
}

// pub inventory: Vec<Item, Global>

pub fn equip_recruit_inventory(
    selected_recruit: &mut ResMut<SelectedRecruit>,
    item: Option<Item>,
    player_stats: &mut ResMut<PlayerStats>,
) -> bool {
    match item {
        Some(Item::Weapon(weapon)) => {
            // Add the weapon to the selected recruit inventory weapon slot
            // + Delete from the player_stat inventory

            let recruit_actual_weapon = selected_recruit
                .0
                .as_ref()
                .unwrap()
                .recruit_inventory
                .weapon
                .iter()
                .find(|w| w.id == weapon.id);

            if !recruit_actual_weapon.is_some() {
                selected_recruit
                    .0
                    .as_mut()
                    .unwrap()
                    .recruit_inventory
                    .weapon = Some(weapon);
            }
        }
        Some(Item::Armor(armor)) => {}
        Some(Item::Scroll(scroll, quantity)) => {}
        None => {}
    }

    return false;
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
