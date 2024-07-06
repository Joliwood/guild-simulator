#![allow(dead_code)]

use bevy::{ecs::component::Tick, prelude::DetectChanges};

pub enum TroopEnum {
    Archer,
    Mage,
    Thief,
    Warrior,
}

#[derive(Debug, PartialEq)]
/// # Where the player is
/// ## (the player can navigate between rooms with arrows) :
///
/// - Office : Main place, at our desk with a top view
/// - Store : The counter when player can buy stuff from a marchand (once at a time, with a CD), or sell stuff
/// - Barrack : Where the troups are, can equip them with stuff, check their level, hp, etc
/// - CommandRoom : Where the player can send troups to missions, check the missions, etc
pub enum RoomEnum {
    Barrack,
    CommandRoom,
    Office,
    Store,
}

impl DetectChanges for RoomEnum {
    fn is_added(&self) -> bool {
        true
    }

    fn last_changed(&self) -> Tick {
        return Tick::new(1);
    }

    fn is_changed(&self) -> bool {
        true
    }
}
