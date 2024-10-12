#![allow(dead_code)]
use super::equipments::{Armor, Scroll, Weapon};
use crate::data::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum};
use bevy::prelude::{Component, Resource};
// ! For workflow with ron files
// use ron::de::from_str;
use uuid::Uuid;

#[derive(Resource)]
pub struct MissionNotificationsNumber(pub u8);

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct MissionReportsModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

// #[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct MissionReport {
//     pub recruit_id: Uuid,
//     pub mission_id: Uuid,
//     pub success: bool,
//     pub experience_gained: Option<u32>,
//     pub golds_gained: Option<i32>,
// }

// #[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct SelectedMission {
//     pub mission: Option<Mission>,
//     pub percent_of_victory: Option<u32>,
//     pub recruit_id: Option<Uuid>,
// }

// impl SelectedMission {
//     pub fn get_mission(&self) -> Option<Mission> {
//         if let Some(mission) = &self.mission {
//             return Some(mission.clone());
//         }

//         None
//     }
// }

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct MissionReport {
    pub recruit_id: Uuid,
    pub mission_id: Uuid,
    pub success: bool,
    pub experience_gained: Option<u32>,
    pub golds_gained: Option<i32>,
}

// #[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct SelectedRecruit(pub Option<RecruitStats>);

// #[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct SelectedMission {
//     pub mission: Option<Mission>,
//     pub percent_of_victory: Option<u32>,
//     pub recruit_id: Option<Uuid>,
// }

// impl SelectedMission {
//     pub fn get_mission(&self) -> Option<Mission> {
//         if let Some(mission) = &self.mission {
//             return Some(mission.clone());
//         }

//         None
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub name: String,
    pub strength: u16,
    pub image_atlas_index: u16,
}

// #[derive(Debug, Component, Resource)]
// pub struct Missions(pub Vec<Mission>);

// impl Missions {
//     pub fn get_mission_by_id(&self, id: Uuid) -> Option<Mission> {
//         if let Some(mission) = self.0.iter().find(|mission| mission.id == id) {
//             return Some(mission.clone());
//         }
//         None
//     }

//     pub fn assign_recruit_id_to_mission(&mut self, mission_id: Uuid, recruit_id: Uuid) {
//         if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
//             mission.assign_recruit_by_id(recruit_id);
//         }
//     }

//     pub fn decrement_days_left_by_mission_id(&mut self, mission_id: Uuid) {
//         if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
//             mission.decrement_days_left();
//         }
//     }

//     pub fn attribute_days_left_to_mission(&mut self, mission_id: Uuid) {
//         if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
//             mission.attribute_days_left();
//         }
//     }

//     pub fn is_mission_over(&self, mission_id: Uuid) -> bool {
//         if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
//             return mission.days_left.is_none();
//         }
//         false
//     }

//     pub fn get_recruit_send_id_by_mission_id(&self, mission_id: Uuid) -> Option<Uuid> {
//         if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
//             return mission.recruit_send;
//         }
//         None
//     }

//     pub fn desassign_recruit_to_mission(&mut self, mission_id: Uuid) {
//         if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
//             mission.desassign_recruit();
//         }
//     }

//     pub fn get_mission_enemmy_level_by_id(&self, mission_id: Uuid) -> Option<u8> {
//         if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
//             return Some(mission.ennemy.level);
//         }
//         None
//     }
// }

// #[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct Mission {
//     pub days: u8,
//     pub ennemy: Ennemy,
//     pub id: Uuid,
//     pub level: u8,
//     pub name: String,
//     pub recruit_send: Option<Uuid>,
//     pub days_left: Option<u8>,
// }

// impl Mission {
//     pub fn decrement_days_left(&mut self) {
//         if let Some(days_left) = &mut self.days_left {
//             if *days_left == 1 {
//                 return self.days_left = None;
//             }
//             if *days_left > 0 {
//                 return *days_left -= 1;
//             }
//         }
//     }

//     pub fn assign_recruit_by_id(&mut self, recruit_id: Uuid) {
//         self.recruit_send = Some(recruit_id);
//     }

//     pub fn desassign_recruit(&mut self) {
//         self.recruit_send = None;
//     }

//     pub fn attribute_days_left(&mut self) {
//         self.days_left = Some(self.days);
//     }
// }

// impl SelectedRecruit {
//     pub fn get_inventory(&self) -> RecruitInventory {
//         if let Some(recruit) = &self.0 {
//             return recruit.recruit_inventory.clone();
//         }
//     }
// }

//         RecruitInventory::generate_empty_inventory()
//     }

//     pub fn get_id(&self) -> Option<Uuid> {
//         if let Some(recruit) = &self.0 {
//             return Some(recruit.id);
//         }

//         None
//     }

//     pub fn equip_weapon(&mut self, weapon: Weapon) {
//         if let Some(recruit) = &mut self.0 {
//             recruit.recruit_inventory.weapon = Some(weapon);
//         }
//     }
// }

// impl PlayerStats {
//     pub fn increment_golds(&mut self, amount: i32) {
//         self.golds += amount;
//     }
// }

//         // Reset the experience with left experience after leveling up
//         // Then level up
//         if self.experience >= self.max_experience {
//             self.experience -= self.max_experience;
//             self.level_up();
//         }
//     }

//     pub fn level_up(&mut self) {
//         self.level += 1;
//         // Set the max experience to the current experience * 2
//         self.max_experience *= 2;
//     }

//     pub fn get_item(&self, item: Item) -> Option<Item> {
//         match item {
//             Item::Weapon(_weapon) => {
//                 if let Some(weapon) = &self.recruit_inventory.weapon {
//                     return Some(Item::Weapon(weapon.clone()));
//                 }
//             }
//             Item::Armor(_armor) => {
//                 if let Some(armor) = &self.recruit_inventory.armor {
//                     return Some(Item::Armor(armor.clone()));
//                 }
//             }
//             Item::Scroll(_scroll, _) => {
//                 if let Some(scroll) = self.recruit_inventory.scrolls.first() {
//                     return Some(Item::Scroll(scroll.clone(), 1));
//                 }
//             }
//         }

//         None
//     }

//     pub fn equip_item(&mut self, item: &Item) {
//         match item {
//             Item::Weapon(weapon) => {
//                 self.recruit_inventory.weapon = Some(weapon.clone());
//             }
//             Item::Armor(armor) => {
//                 self.recruit_inventory.armor = Some(armor.clone());
//             }
//             Item::Scroll(scroll, _) => {
//                 self.recruit_inventory.scrolls.push(scroll.clone());
//             }
//         }
//     }

// pub fn add_item(&mut self, item: Item) {
//     match item {
//         Item::Scroll(scroll, quantity) => {
//             let scroll_id = scroll.id;
//             if self.inventory.iter().any(|item| match item {
//                 Item::Scroll(scroll, _) => scroll.id == scroll_id,
//                 _ => false,
//             }) {
//                 self.inventory.iter_mut().for_each(|item| {
//                     if let Item::Scroll(scroll, q) = item {
//                         if scroll.id == scroll_id {
//                             *q += quantity;
//                         }
//                     }
//                 });
//             } else {
//                 self.inventory.push(Item::Scroll(scroll, quantity));
//             }
//         }
//         _ => {
//             if self.inventory.len() < self.max_inventory_size {
//                 self.inventory.push(item);
//             }
//         }
//     }
//     // self.inventory.push(item);
// }

//         if let Some(weapon) = &self.recruit_inventory.weapon {
//             if let Some(strength) = weapon.strength {
//                 additional_strength += strength;
//             }
//         }

//         if let Some(armor) = &self.recruit_inventory.armor {
//             if let Some(strength) = armor.strength {
//                 additional_strength += strength;
//             }
//         }

// pub fn equip_item_to_recruit(&mut self, recruit_id: Uuid, item: &Item) {
//     if let Some(recruit) = self
//         .recruits
//         .iter_mut()
//         .find(|recruit| recruit.id == recruit_id)
//     {
//         recruit.equip_item(item);
//     }
// }

//         additional_strength
//     }

//     pub fn get_additional_endurance_from_items(&self) -> u32 {
//         let mut additional_endurance = 0;

//         if let Some(weapon) = &self.recruit_inventory.weapon {
//             if let Some(endurance) = weapon.endurance {
//                 additional_endurance += endurance;
//             }
//         }

//         if let Some(armor) = &self.recruit_inventory.armor {
//             if let Some(endurance) = armor.endurance {
//                 additional_endurance += endurance;
//             }
//         }

//         for scroll in &self.recruit_inventory.scrolls {
//             if let Some(endurance) = scroll.endurance {
//                 additional_endurance += endurance;
//             }
//         }

//         additional_endurance
//     }

//     pub fn get_additional_intelligence_from_items(&self) -> u32 {
//         let mut additional_intelligence = 0;

//         if let Some(weapon) = &self.recruit_inventory.weapon {
//             if let Some(intelligence) = weapon.intelligence {
//                 additional_intelligence += intelligence;
//             }
//         }

//         if let Some(armor) = &self.recruit_inventory.armor {
//             if let Some(intelligence) = armor.intelligence {
//                 additional_intelligence += intelligence;
//             }
//         }

//         for scroll in &self.recruit_inventory.scrolls {
//             if let Some(intelligence) = scroll.intelligence {
//                 additional_intelligence += intelligence;
//             }
//         }

//         additional_intelligence
//     }

//     pub fn get_total_merged_stats(&self) -> u32 {
//         return self.strength as u32
//             + self.get_additional_strength_from_items()
//             + self.endurance as u32
//             + self.get_additional_endurance_from_items()
//             + self.intelligence as u32
//             + self.get_additional_intelligence_from_items();
//     }
// }

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_weapon_by_id(id: u16) -> Option<Weapon> {
//     let weapons_data = fs::read_to_string("src/data/equipments/weapons.ron")
//         .expect("Failed to read the RON file.");

//     let weapons: Weapons = from_str(&weapons_data).expect("Failed to deserialize RON data.");

//     if let Some(weapon) = weapons.items.iter().find(|weapon| weapon.id == id) {
//         info!("Weapon with id = {}: {:?}", id, weapon);
//         return Some(weapon.clone());
//     } else {
//         info!("Weapon with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_weapon(weapon: WeaponsEnum) -> Weapon {
    return WeaponsEnum::get_weapon(&weapon);
}

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_scroll_by_id(id: u16) -> Option<Scroll> {
//     let scrolls_data = fs::read_to_string("src/data/equipments/scrolls.ron")
//         .expect("Failed to read the RON file.");

//     let scrolls: Scrolls = from_str(&scrolls_data).expect("Failed to deserialize RON data.");

//     if let Some(scroll) = scrolls.items.iter().find(|scroll| scroll.id == id) {
//         info!("Scroll with id = {}: {:?}", id, scroll);
//         return Some(scroll.clone());
//     } else {
//         info!("Scroll with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_scroll(scroll: ScrollsEnum) -> Scroll {
    return ScrollsEnum::get_scroll(&scroll);
}

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_armor_by_id(id: u16) -> Option<Armor> {
//     let armors_data =
//         fs::read_to_string("src/data/equipments/armors.ron").expect("Failed to read the RON file.");

//     let armors: Armors = from_str(&armors_data).expect("Failed to deserialize RON data.");

//     if let Some(armor) = armors.items.iter().find(|armor| armor.id == id) {
//         info!("Armor with id = {}: {:?}", id, armor);
//         return Some(armor.clone());
//     } else {
//         info!("Armor with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_armor(armor: ArmorsEnum) -> Armor {
    return ArmorsEnum::get_armor(&armor);
}

// impl Default for PlayerStats {
//     fn default() -> Self {
//         let mut inventory = vec![];
//         let first_weapon = load_weapon(WeaponsEnum::AxeOfFury);
//         let second_weapon = load_weapon(WeaponsEnum::MaceOfTheThunder);
//         let second_same_weapon = load_weapon(WeaponsEnum::MaceOfTheThunder);
//         let first_scroll = load_scroll(ScrollsEnum::ScrollOfEndurance);
//         let second_scroll = load_scroll(ScrollsEnum::ScrollOfSpeed);
//         let first_armor = load_armor(ArmorsEnum::GauntletsOfPower);
//         let second_armor = load_armor(ArmorsEnum::HelmetOfTheGuardian);
//         let second_same_armor = load_armor(ArmorsEnum::HelmetOfTheGuardian);

//         inventory.push(Item::Weapon(first_weapon));
//         inventory.push(Item::Weapon(second_weapon));
//         inventory.push(Item::Weapon(second_same_weapon));
//         inventory.push(Item::Scroll(first_scroll, 1));
//         inventory.push(Item::Scroll(second_scroll, 3));
//         inventory.push(Item::Armor(first_armor));
//         inventory.push(Item::Armor(second_armor));
//         inventory.push(Item::Armor(second_same_armor));

//         Self {
//             day: 1,
//             experience: 0,
//             golds: 0,
//             guild_level: 1,
//             inventory,
//             max_experience: 100,
//             max_inventory_size: 50,
//             recruits: vec![],
//             room: RoomEnum::Office,
//         }
//     }
// }

// impl Default for Missions {
//     fn default() -> Self {
//         Self(vec![
//             Mission {
//                 days_left: None,
//                 days: 1,
//                 id: Uuid::new_v4(),
//                 level: 1,
//                 name: "Mission 1".to_string(),
//                 recruit_send: None,
//                 ennemy: Ennemy {
//                     endurance: 10,
//                     experience: 0,
//                     intelligence: 5,
//                     level: 1,
//                     name: "Ennemy 1".to_string(),
//                     strength: 10,
//                 },
//             },
//             Mission {
//                 days_left: None,
//                 days: 1,
//                 id: Uuid::new_v4(),
//                 level: 2,
//                 name: "Mission 2".to_string(),
//                 recruit_send: None,
//                 ennemy: Ennemy {
//                     endurance: 15,
//                     experience: 0,
//                     intelligence: 7,
//                     level: 2,
//                     name: "Ennemy 2".to_string(),
//                     strength: 15,
//                 },
//             },
//             Mission {
//                 days_left: Some(1),
//                 days: 2,
//                 id: Uuid::new_v4(),
//                 level: 3,
//                 name: "Mission 3".to_string(),
//                 recruit_send: None,
//                 ennemy: Ennemy {
//                     endurance: 20,
//                     experience: 0,
//                     intelligence: 10,
//                     level: 3,
//                     name: "Ennemy 3".to_string(),
//                     strength: 20,
//                 },
//             },
//         ])
//     }
// }
