#![allow(dead_code)]

use crate::enums::RoomEnum;
use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Component)]
pub struct GoldCountText;

#[derive(Resource)]
pub struct PlayerStats {
    pub golds: i32,
    pub troups_count: i32,
    pub room: RoomEnum,
}

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        println!(
            "Incrementing golds by {} for a total of : {}",
            amount, self.golds,
        );
        self.golds += amount;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            golds: 5,
            troups_count: 0,
            room: RoomEnum::Office,
        }
    }
}
