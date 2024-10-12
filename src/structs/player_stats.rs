#![allow(dead_code)]
use super::{
    equipments::Item,
    general_structs::{load_armor, load_scroll, load_weapon},
    recruits::RecruitStats,
};
use crate::{
    data::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum},
    enums::{RecruitStateEnum, RoomEnum},
};
use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Resource, Clone)]
pub struct PlayerStats {
    pub day: u16,
    pub experience: u32,
    pub golds: i32,
    pub guild_level: i8,
    pub inventory: Vec<Item>,
    pub max_experience: u32,
    pub max_inventory_size: usize,
    pub recruits: Vec<RecruitStats>,
    pub room: RoomEnum,
}

impl Default for PlayerStats {
    fn default() -> Self {
        let mut inventory = vec![];
        let first_weapon = load_weapon(WeaponsEnum::AxeOfFury);
        let second_weapon = load_weapon(WeaponsEnum::MaceOfTheThunder);
        let second_same_weapon = load_weapon(WeaponsEnum::MaceOfTheThunder);
        let first_scroll = load_scroll(ScrollsEnum::ScrollOfEndurance);
        let second_scroll = load_scroll(ScrollsEnum::ScrollOfSpeed);
        let first_armor = load_armor(ArmorsEnum::GauntletsOfPower);
        let second_armor = load_armor(ArmorsEnum::HelmetOfTheGuardian);
        let second_same_armor = load_armor(ArmorsEnum::HelmetOfTheGuardian);

        inventory.push(Item::Weapon(first_weapon));
        inventory.push(Item::Weapon(second_weapon));
        inventory.push(Item::Weapon(second_same_weapon));
        inventory.push(Item::Scroll(first_scroll, 1));
        inventory.push(Item::Scroll(second_scroll, 3));
        inventory.push(Item::Armor(first_armor));
        inventory.push(Item::Armor(second_armor));
        inventory.push(Item::Armor(second_same_armor));

        Self {
            day: 1,
            experience: 0,
            golds: 0,
            guild_level: 1,
            inventory,
            max_experience: 100,
            max_inventory_size: 50,
            recruits: vec![],
            room: RoomEnum::Office,
        }
    }
}

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        self.golds += amount;
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.guild_level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }

    pub fn find_item_by_id(&self, id: u16) -> Option<Item> {
        if let Some(item) = self.inventory.iter().find(|item| match item {
            Item::Weapon(weapon) => weapon.id == id,
            Item::Armor(armor) => armor.id == id,
            Item::Scroll(scroll, _) => scroll.id == id,
        }) {
            return Some(item.clone());
        }

        None
    }

    pub fn add_item(&mut self, item: Item) {
        match item {
            Item::Scroll(scroll, quantity) => {
                let scroll_id = scroll.id;
                if self.inventory.iter().any(|item| match item {
                    Item::Scroll(scroll, _) => scroll.id == scroll_id,
                    _ => false,
                }) {
                    self.inventory.iter_mut().for_each(|item| {
                        if let Item::Scroll(scroll, q) = item {
                            if scroll.id == scroll_id {
                                *q += quantity;
                            }
                        }
                    });
                } else {
                    self.inventory.push(Item::Scroll(scroll, quantity));
                }
            }
            _ => {
                if self.inventory.len() < self.max_inventory_size {
                    self.inventory.push(item);
                }
            }
        }
        // self.inventory.push(item);
    }

    pub fn get_recruit_by_id(&self, id: Uuid) -> Option<RecruitStats> {
        if let Some(recruit) = self.recruits.iter().find(|recruit| recruit.id == id) {
            return Some(recruit.clone());
        }

        None
    }

    pub fn equip_item_to_recruit(&mut self, recruit_id: Uuid, item: &Item) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.equip_item(item);
        }
    }

    pub fn gain_xp_to_recruit(&mut self, recruit_id: Uuid, xp: u32) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.gain_xp(xp);
        }
    }

    pub fn remove_one_scroll_from_inventory(&mut self, scroll_id: u16) {
        if let Some(scroll_index) = self.inventory.iter().position(|item| match item {
            Item::Scroll(scroll, _) => scroll.id == scroll_id,
            _ => false,
        }) {
            if let Item::Scroll(_scroll, quantity) = &mut self.inventory[scroll_index] {
                if *quantity > 1 {
                    *quantity -= 1;
                } else {
                    self.inventory.remove(scroll_index);
                }
            }
        }
    }

    pub fn update_state_of_recruit(&mut self, recruit_id: Uuid, state: RecruitStateEnum) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.state = state;
        }
    }

    // Importée de general structs
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

    //     pub fn equip_item_to_recruit(&mut self, recruit_id: Uuid, item: &Item) {
    //         if let Some(recruit) = self
    //             .recruits
    //             .iter_mut()
    //             .find(|recruit| recruit.id == recruit_id)
    //         {
    //             recruit.equip_item(item);
    //         }
    //     }
}
