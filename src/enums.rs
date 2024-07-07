#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum RecruitEnum {
    Hunter,
    Mage,
    Rogue,
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

#[derive(Debug, PartialEq)]
pub enum RoomDirectionEnum {
    Bottom,
    Left,
    Right,
    Top,
}
