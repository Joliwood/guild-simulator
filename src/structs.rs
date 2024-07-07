#![allow(dead_code)]

use crate::enums::{RecruitEnum, RoomEnum};
use bevy::{
    ecs::component::Tick,
    prelude::{Component, Resource},
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
    pub recruits: Vec<RecruitStats>,
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

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            golds: 5,
            room: RoomEnum::Office,
            recruits: vec![],
        }
    }
}

#[derive(Debug, Component)]
pub struct RecruitStats {
    pub class: RecruitEnum,
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub max_experience: u32,
    pub strength: u16,
}
