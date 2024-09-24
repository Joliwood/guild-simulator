#![allow(dead_code)]

use bevy::{prelude::Component, state::state::States};
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum RecruitEnum {
    Hunter,
    Mage,
    Rogue,
    Warrior,
}

// #[derive(Component, Debug, PartialEq)]
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Component)]
///  Where the player is
///  (the player can navigate between rooms with arrows) :
///
/// - Office : Main place, at our desk with a top view
/// - Store : The counter when player can buy stuff from a marchand (once at a time, with a CD), or sell stuff
/// - Barrack : Where the troups are, can equip them with stuff, check their level, hp, etc
/// - CommandRoom : Where the player can send troups to missions, check the missions, etc
pub enum RoomEnum {
    Barrack,
    CommandRoom,
    #[default]
    Office,
    Store,
}

#[derive(Debug, PartialEq)]
pub enum RoomDirectionEnum {
    Bottom,
    Left,
    Right,
    Top,
}

impl Display for RoomEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for RecruitEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ItemRaretyEnum {
    Common,
    UnCommon,
    Rare,
}
