#![allow(dead_code)]

use bevy::{color::Color, prelude::Component, state::state::States};
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

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum ItemRaretyEnum {
    Common,
    UnCommon,
    Rare,
}

#[derive(Debug, Clone)]
pub enum SoundEnum {
    BookThrowDown,
    CockrelMorning,
    EquipArmor,
    EquipmentEquip,
    EquipScroll,
    EquipWeapon,
    KeysRemovedFromDoor,
    PaperTouch,
    SimpleHolidaysV3,
}

impl SoundEnum {
    pub fn get_path(&self) -> &'static str {
        match self {
            SoundEnum::BookThrowDown => "sounds/book_throw_down.ogg",
            SoundEnum::CockrelMorning => "sounds/cockrel_morning.ogg",
            SoundEnum::EquipArmor => "sounds/equip_armor.ogg",
            SoundEnum::EquipmentEquip => "sounds/equipment_equip.ogg",
            SoundEnum::EquipScroll => "sounds/equip_scroll.ogg",
            SoundEnum::EquipWeapon => "sounds/equip_weapon.ogg",
            SoundEnum::KeysRemovedFromDoor => "sounds/keys_removed_from_door.ogg",
            SoundEnum::PaperTouch => "sounds/paper_touch.ogg",
            SoundEnum::SimpleHolidaysV3 => "sounds/Simple-Holidays-V3.ogg",
        }
    }
}

#[derive(Debug)]
pub enum ColorPaletteEnum {
    Danger,
    DarkBrown,
    Info,
    Primary,
    Success,
    Warning,
}

impl ColorPaletteEnum {
    pub fn as_color(&self) -> Color {
        match *self {
            ColorPaletteEnum::Danger => Color::srgb(0.9, 0.1, 0.1),
            ColorPaletteEnum::DarkBrown => Color::srgb(0.13, 0.11, 0.09),
            ColorPaletteEnum::Info => Color::srgb(0.2, 0.7, 0.9),
            ColorPaletteEnum::Primary => Color::srgb(0.1, 0.2, 0.8),
            ColorPaletteEnum::Success => Color::srgb(0.1, 0.8, 0.1),
            ColorPaletteEnum::Warning => Color::srgb(0.9, 0.7, 0.0),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum RecruitStateEnum {
    Available,
    InMission,
    InRecuperation,
    Injured,
}
