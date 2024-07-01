#![allow(dead_code)]

pub enum TroopEnum {
    Archer,
    Mage,
    Thief,
    Warrior,
}

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
