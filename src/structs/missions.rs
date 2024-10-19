use super::{
    equipments::ItemEnum,
    general_structs::{load_armor, load_scroll, load_weapon, Ennemy},
    player_stats::PlayerStats,
};
use crate::{
    data::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum},
    utils::{calculate_price_range, get_global_points, get_victory_percentage},
};
use bevy::prelude::*;
use uuid::Uuid;

#[derive(Default, Debug, Component, Resource)]
pub struct MissionReports(pub Vec<MissionReport>);

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct MissionReport {
    pub experience_gained: Option<u32>,
    pub golds_gained: Option<i32>,
    pub loots: Vec<ItemEnum>,
    pub mission_id: u16,
    pub mission_ids_to_unlock: Vec<u16>,
    pub percent_of_victory: u32,
    pub recruit_id: Uuid,
    pub success: bool,
}

impl MissionReport {
    pub fn add_item(&mut self, item: ItemEnum) {
        match item {
            ItemEnum::Scroll(scroll, quantity) => {
                let scroll_id = scroll.id;
                if self.loots.iter().any(|item| match item {
                    ItemEnum::Scroll(scroll, _) => scroll.id == scroll_id,
                    _ => false,
                }) {
                    self.loots.iter_mut().for_each(|item| {
                        if let ItemEnum::Scroll(scroll, q) = item {
                            if scroll.id == scroll_id {
                                *q += quantity;
                            }
                        }
                    });
                } else {
                    self.loots.push(ItemEnum::Scroll(scroll, quantity));
                }
            }
            _ => {
                self.loots.push(item);
            }
        }
        // self.inventory.push(item);
    }

    pub fn calculate_loots(&mut self, loots: Loots) {
        let item_loots = loots.0;

        if item_loots.is_empty() {
            return;
        }

        // Helper function to add item based on its enum type
        let mut add_item_to_inventory = |item_loot: &ItemLoot| match &item_loot.item {
            ItemLootEnum::Armor(armor) => {
                let armor = load_armor(armor.clone());
                self.add_item(ItemEnum::Armor(armor));
            }
            ItemLootEnum::Scroll(scroll) => {
                let scroll = load_scroll(scroll.clone());
                self.add_item(ItemEnum::Scroll(scroll, 1));
            }
            ItemLootEnum::Weapon(weapon) => {
                let weapon = load_weapon(weapon.clone());
                self.add_item(ItemEnum::Weapon(weapon));
            }
        };

        // Step 1: Pick one guaranteed random item
        let first_random_item_index = rand::random::<usize>() % item_loots.len();
        let first_item = &item_loots[first_random_item_index];
        add_item_to_inventory(first_item);

        // Step 2: 50% chance to pick a second item (can be same or different)
        let second_chance = rand::random::<u8>() % 100;
        if second_chance < 50 {
            let first_random_item_index = rand::random::<usize>() % item_loots.len();
            let second_item = &item_loots[first_random_item_index];
            add_item_to_inventory(second_item);
        }
    }
}

impl MissionReports {
    pub fn add_mission_report(&mut self, report: MissionReport) {
        self.0.push(report);
    }

    pub fn get_mission_report_by_id(&self, mission_id: u16) -> Option<MissionReport> {
        if let Some(report) = self.0.iter().find(|report| report.mission_id == mission_id) {
            return Some(report.clone());
        }
        None
    }

    pub fn get_last_mission_report(&self) -> Option<MissionReport> {
        if let Some(report) = self.0.last() {
            return Some(report.clone());
        }
        None
    }

    pub fn remove_mission_report_by_id(&mut self, mission_id: u16) {
        if let Some(index) = self
            .0
            .iter()
            .position(|report| report.mission_id == mission_id)
        {
            self.0.remove(index);
        }
    }

    // pub fn add_loots_by_item_loot_by_mission_id(&mut self, mission_id: u16, loots: Loots) {
    //     if let Some(report) = self
    //         .0
    //         .iter_mut()
    //         .find(|report| report.mission_id == mission_id)
    //     {
    //         report.loots = loots.0;
    //     }
    // }
}

// pub fn add_loots_to_inventory_by_item_loot(&mut self, loots: Loots) {
//     let item_loots = loots.0;

//     if item_loots.is_empty() {
//         return;
//     }

//     // Helper function to add item based on its enum type
//     let mut add_item_to_inventory = |item_loot: &ItemLoot| match &item_loot.item {
//         ItemLootEnum::Armor(armor) => {
//             let armor = load_armor(armor.clone());
//             self.add_item(ItemEnum::Armor(armor));
//         }
//         ItemLootEnum::Scroll(scroll) => {
//             let scroll = load_scroll(scroll.clone());
//             self.add_item(ItemEnum::Scroll(scroll, 1));
//         }
//         ItemLootEnum::Weapon(weapon) => {
//             let weapon = load_weapon(weapon.clone());
//             self.add_item(ItemEnum::Weapon(weapon));
//         }
//     };

//     // Step 1: Pick one guaranteed random item
//     let first_random_item_index = rand::random::<usize>() % item_loots.len();
//     let first_item = &item_loots[first_random_item_index];
//     add_item_to_inventory(first_item);

//     // Step 2: 50% chance to pick a second item (can be same or different)
//     let second_chance = rand::random::<u8>() % 100;
//     if second_chance < 50 {
//         let first_random_item_index = rand::random::<usize>() % item_loots.len();
//         let second_item = &item_loots[first_random_item_index];
//         add_item_to_inventory(second_item);
//     }
// }

#[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedMission {
    pub mission: Option<Mission>,
    pub percent_of_victory: Option<u32>,
    pub recruit_id: Option<Uuid>,
}

impl SelectedMission {
    pub fn get_mission(&self) -> Option<Mission> {
        if let Some(mission) = &self.mission {
            return Some(mission.clone());
        }

        None
    }

    pub fn calculate_percent_of_victory(&mut self, player_stats: &Res<PlayerStats>) {
        if self.mission.is_none() {
            return;
        }

        let ennemy = self.mission.as_ref().unwrap().ennemy.clone();
        let ennemy_global_points =
            get_global_points(ennemy.strength, ennemy.endurance, ennemy.intelligence);

        let recruit_id = self.recruit_id.unwrap();
        let recruit = player_stats.get_recruit_by_id(recruit_id).unwrap();
        let recruit_global_points = recruit.get_total_merged_stats();

        let victory_percentage =
            get_victory_percentage(recruit_global_points as u16, ennemy_global_points) as u32;

        self.percent_of_victory = Some(victory_percentage);

        // let recruit = self.percent_of_victory = Some(get_victory_percentage(
        //     recruit_global_points,
        //     ennemy_global_points,
        // ));
    }
}

#[derive(Debug, Component, Resource)]
pub struct Missions(pub Vec<Mission>);

impl Missions {
    pub fn get_mission_by_id(&self, id: u16) -> Option<Mission> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == id) {
            return Some(mission.clone());
        }
        None
    }

    pub fn get_mission_by_name(&self, name: &str) -> Option<Mission> {
        return self.0.iter().find(|mission| mission.name == name).cloned();
    }

    pub fn assign_recruit_id_to_mission(&mut self, mission_id: u16, recruit_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.assign_recruit_by_id(recruit_id);
        }
    }

    pub fn decrement_days_left_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.decrement_days_left();
        }
    }

    pub fn attribute_days_left_to_mission(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.attribute_days_left();
        }
    }

    pub fn is_mission_over(&self, mission_id: u16) -> bool {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.days_left.is_none();
        }
        false
    }

    pub fn get_recruit_send_id_by_mission_id(&self, mission_id: u16) -> Option<Uuid> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.recruit_send;
        }
        None
    }

    pub fn desassign_recruit_to_mission(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.desassign_recruit();
        }
    }

    pub fn get_mission_enemmy_level_by_id(&self, mission_id: u16) -> Option<u8> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.ennemy.level);
        }
        None
    }

    pub fn get_percent_of_victory_by_mission_id(&self, mission_id: u16) -> Option<u32> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.percent_of_victory;
        }
        None
    }

    pub fn attribute_percent_of_victory_to_mission(
        &mut self,
        mission_id: u16,
        percent_of_victory: u32,
    ) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.percent_of_victory = Some(percent_of_victory);
        }
    }

    pub fn get_missions_by_ids(&self, ids: Vec<u16>) -> Vec<Mission> {
        let mut missions = vec![];

        for id in ids {
            if let Some(mission) = self.get_mission_by_id(id) {
                missions.push(mission);
            }
        }

        missions
    }

    pub fn get_golds_earned_by_mission_id(&self, mission_id: u16) -> Option<u32> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.golds);
        }
        None
    }

    pub fn unlock_missions_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            for id in mission.unlock_mission_ids.clone() {
                if let Some(mission_to_unlock) = self.0.iter_mut().find(|mission| mission.id == id)
                {
                    mission_to_unlock.unlocked = true;
                }
            }
        }
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub enum ItemLootEnum {
    Armor(ArmorsEnum),
    Scroll(ScrollsEnum),
    Weapon(WeaponsEnum),
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct ItemLoot {
    pub item: ItemLootEnum,
    pub percent: u8,
}

impl ItemLoot {
    pub fn get_atlas_index(&self) -> u16 {
        match &self.item {
            ItemLootEnum::Armor(armor) => armor.get_armor().image_atlas_index,
            ItemLootEnum::Scroll(scroll) => scroll.get_scroll().image_atlas_index,
            ItemLootEnum::Weapon(weapon) => weapon.get_weapon().image_atlas_index,
        }
    }

    pub fn get_item_layout(&self) -> TextureAtlasLayout {
        return match self {
            ItemLoot {
                item: ItemLootEnum::Armor(_),
                ..
            } => TextureAtlasLayout::from_grid(
                UVec2::new(1600, 400),
                4,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
            ItemLoot {
                item: ItemLootEnum::Scroll(_),
                ..
            } => TextureAtlasLayout::from_grid(
                UVec2::new(4320, 1080),
                4,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
            ItemLoot {
                item: ItemLootEnum::Weapon(_),
                ..
            } => TextureAtlasLayout::from_grid(
                UVec2::new(2900, 400),
                6,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
        };
    }

    pub fn get_item_loot_tooltip_description(&self) -> String {
        return match self {
            ItemLoot {
                item: ItemLootEnum::Armor(armor),
                ..
            } => {
                let armor = armor.get_armor();
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
            ItemLoot {
                item: ItemLootEnum::Scroll(scroll),
                ..
            } => {
                let scroll = scroll.get_scroll();
                let mut description = format!("{}\n", scroll.name);
                let price_range = calculate_price_range(scroll.price);

                if let Some(endurance) = scroll.endurance {
                    description.push_str(&format!("\nEndurance: {}", endurance));
                }
                if let Some(strength) = scroll.strength {
                    description.push_str(&format!("\nStrength: {}", strength));
                }
                if let Some(intelligence) = scroll.intelligence {
                    description.push_str(&format!("\nIntelligence: {}", intelligence));
                }
                description.push_str(&format!(
                    "\n\nPrice: {} to {} G",
                    price_range.0, price_range.1
                ));

                description
            }
            ItemLoot {
                item: ItemLootEnum::Weapon(weapon),
                ..
            } => {
                let weapon = weapon.get_weapon();
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
        };
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Loots(pub Vec<ItemLoot>);

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub days_left: Option<u8>,
    pub days: u8,
    pub description: String,
    pub ennemy: Ennemy,
    pub golds: u32,
    pub id: u16,
    pub level: u8,
    pub loots: Loots,
    pub name: String,
    pub percent_of_victory: Option<u32>,
    pub recruit_send: Option<Uuid>,
    pub unlock_mission_ids: Vec<u16>,
    pub unlocked: bool,
}

impl Mission {
    pub fn decrement_days_left(&mut self) {
        if let Some(days_left) = &mut self.days_left {
            if *days_left == 1 {
                return self.days_left = None;
            }
            if *days_left > 0 {
                return *days_left -= 1;
            }
        }
    }

    pub fn assign_recruit_by_id(&mut self, recruit_id: Uuid) {
        self.recruit_send = Some(recruit_id);
    }

    pub fn desassign_recruit(&mut self) {
        self.recruit_send = None;
    }

    pub fn attribute_days_left(&mut self) {
        self.days_left = Some(self.days);
    }
}

impl Default for Missions {
    fn default() -> Self {
        Self(vec![
            Mission {
                days_left: None,
                days: 1,
                id: 1,
                level: 1,
                name: "Mission 1".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 5,
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    name: "Ennemy 1".to_string(),
                    strength: 10,
                },
                unlocked: true,
                description: "A basic camp, we think we could find some resources here. We need to send a recruit to check it out."
                .to_string(),
                golds: 50,
                loots: Loots(
                    vec![
                        ItemLoot {
                            item: ItemLootEnum::Weapon(WeaponsEnum::SwordOfValor),
                            percent: 50,
                        },
                        ItemLoot {
                            item: ItemLootEnum::Armor(ArmorsEnum::GauntletsOfPower),
                            percent: 50,
                        },
                        ItemLoot {
                            item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfWisdom),
                            percent: 50,
                        },
                    ]
                ),
                unlock_mission_ids: vec![2],
            },
            Mission {
                days_left: None,
                days: 1,
                id: 2,
                level: 2,
                name: "Mission 2".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 4,
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    name: "Ennemy 2".to_string(),
                    strength: 15,
                },
                unlocked: false,
                description: "More extended camp, we think we could find some resources here. We need to send a recruit to check it out.".to_string(),
                golds: 80,
                loots: Loots(
                    vec![],
                ),
                unlock_mission_ids: vec![3],
            },
            Mission {
                days_left: Some(1),
                days: 2,
                id: 3,
                level: 3,
                name: "Mission 3".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 3,
                    endurance: 20,
                    experience: 0,
                    intelligence: 10,
                    level: 3,
                    name: "Ennemy 3".to_string(),
                    strength: 20,
                },
                unlocked: false,
                description: "A very extended camp, the final one of the tuto".to_string(),
                golds: 150,
                loots: Loots(
                    vec![],
                ),
                unlock_mission_ids: vec![],
            },
        ])
    }
}
