#![allow(dead_code)]

use crate::enums::RoomEnum;
use bevy::{
    ecs::component::Tick,
    prelude::{Component, DetectChanges, Resource},
};

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct PlayerStatsRoomTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

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

impl DetectChanges for PlayerStats {
    fn is_added(&self) -> bool {
        false
    }

    fn last_changed(&self) -> Tick {
        return Tick::new(1);
    }

    fn is_changed(&self) -> bool {
        true
    }
}
