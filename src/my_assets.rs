#![allow(dead_code)]
use crate::{
    enums::{MapImageEnum, SoundEnum},
    structs::{equipments::ItemEnum, missions::ItemLootEnum},
};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // --- Recruits --- //
    #[asset(path = "images/recruits/recruit_picture_atlas.png")]
    pub recruit_picture_atlas: Handle<Image>,
    // #[asset(path = "images/recruits/recruit_card_win_percent.png")]
    // pub recruit_card_win_percent: Handle<Image>,

    // --- Ennemies --- //
    #[asset(path = "images/missions/ennemy_picture_atlas.png")]
    pub ennemy_image_handle: Handle<Image>,

    // --- Equipments --- //
    #[asset(path = "images/equipments/empty_inventory_slot.png")]
    pub empty_inventory_slot: Handle<Image>,
    #[asset(path = "images/equipments/armors_atlas.png")]
    pub armors_atlas: Handle<Image>,
    #[asset(path = "images/equipments/weapons_atlas.png")]
    pub weapons_atlas: Handle<Image>,
    #[asset(path = "images/equipments/scrolls_atlas.png")]
    pub scrolls_atlas: Handle<Image>,

    // --- UI --- //
    #[asset(path = "images/ui/art_v0_buttons.png")]
    pub art_v0_buttons: Handle<Image>,
    #[asset(path = "images/ui/buttons_atlas.png")]
    pub buttons_atlas: Handle<Image>,
    #[asset(path = "images/ui/gold.png")]
    pub gold: Handle<Image>,
    #[asset(path = "images/ui/notification_atlas.png")]
    pub notification_atlas: Handle<Image>,
    #[asset(path = "images/ui/play.png")]
    pub _play: Handle<Image>,

    // --- Rooms > Barrack --- //
    #[asset(path = "images/rooms/barrack/barrack_background.png")]
    pub barrack_background: Handle<Image>,
    #[asset(path = "images/rooms/barrack/inventory_container.png")]
    pub inventory_container: Handle<Image>,
    #[asset(path = "images/rooms/barrack/recruit_frame.png")]
    pub recruit_frame: Handle<Image>,
    #[asset(path = "images/rooms/barrack/recruit_infos.png")]
    pub recruit_infos: Handle<Image>,

    // --- Rooms > Office ---//
    #[asset(path = "images/rooms/office/office_room_background.png")]
    pub office_background: Handle<Image>,
    #[asset(path = "images/rooms/office/desk.png")]
    pub desk: Handle<Image>,
    #[asset(path = "images/rooms/office/mission_notification_document.png")]
    pub mission_notification_document: Handle<Image>,
    #[asset(path = "images/rooms/office/talents_on_desk.png")]
    pub talents_on_desk: Handle<Image>,
    #[asset(path = "images/rooms/office/notification_token_in_wood.png")]
    pub notification_token_in_wood: Handle<Image>,
    #[asset(path = "images/rooms/office/set_of_keys.png")]
    pub set_of_keys: Handle<Image>,
    #[asset(path = "images/rooms/office/set_of_keys_container.png")]
    pub set_of_keys_container: Handle<Image>,

    // --- Rooms > Command room --- //
    #[asset(path = "images/rooms/command_room/command_room_background.png")]
    pub command_room_background: Handle<Image>,
    #[asset(path = "images/rooms/command_room/command_table.png")]
    pub command_table: Handle<Image>,
    #[asset(path = "images/rooms/command_room/wood_box_container.png")]
    pub wood_box_container: Handle<Image>,
    #[asset(path = "images/rooms/command_room/recruit_card.png")]
    pub recruit_card: Handle<Image>,

    // --- Rooms > Store --- //
    #[asset(path = "images/store.png")]
    pub store: Handle<Image>,

    // --- Fonts --- //
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_bold: Handle<Font>,

    // --- Sounds --- //
    #[asset(path = "sounds/Simple-Holidays-V3.ogg")] // Change path as necessary
    pub simple_holidays_v3: Handle<AudioSource>,
    #[asset(path = "sounds/book_throw_down.ogg")] // Add more sounds as needed
    pub book_throw_down: Handle<AudioSource>,
    #[asset(path = "sounds/cockrel_morning.ogg")]
    pub cockrel_morning: Handle<AudioSource>,
    #[asset(path = "sounds/equip_armor.ogg")]
    pub equip_armor: Handle<AudioSource>,
    #[asset(path = "sounds/equipment_equip.ogg")]
    pub equipment_equip: Handle<AudioSource>,
    #[asset(path = "sounds/equip_scroll.ogg")]
    pub equip_scroll: Handle<AudioSource>,
    #[asset(path = "sounds/equip_weapon.ogg")]
    pub equip_weapon: Handle<AudioSource>,
    #[asset(path = "sounds/keys_removed_from_door.ogg")]
    pub keys_removed_from_door: Handle<AudioSource>,
    #[asset(path = "sounds/paper_touch.ogg")]
    pub paper_touch: Handle<AudioSource>,
    #[asset(path = "sounds/pencil_sign.ogg")]
    pub pencil_sign: Handle<AudioSource>,
    #[asset(path = "sounds/picking_golds.ogg")]
    pub picking_golds: Handle<AudioSource>,

    // --- Maps --- //
    #[asset(path = "images/maps/map_tuto.png")]
    pub map_tuto: Handle<Image>,
    #[asset(path = "images/maps/map_description.png")]
    pub map_description: Handle<Image>,
    #[asset(path = "images/maps/map_atlas.png")]
    pub map_atlas: Handle<Image>,
    #[asset(path = "images/maps/map_card.png")]
    pub map_card: Handle<Image>,
    #[asset(path = "images/maps/map_type_atlas.png")]
    pub map_type_atlas: Handle<Image>,
    #[asset(path = "images/maps/limited_time.png")]
    pub limited_time: Handle<Image>,

    // --- Missions --- //
    #[asset(path = "images/missions/c1_mission_1.png")]
    pub c1_mission_1: Handle<Image>,
    #[asset(path = "images/missions/c1_mission_2.png")]
    pub c1_mission_2: Handle<Image>,
    #[asset(path = "images/missions/c1_mission_3.png")]
    pub c1_mission_3: Handle<Image>,
    #[asset(path = "images/missions/c1_mission_4.png")]
    pub c1_mission_4: Handle<Image>,
    #[asset(path = "images/missions/c1_mission_5.png")]
    pub c1_mission_5: Handle<Image>,
    #[asset(path = "images/missions/c1_mission_6.png")]
    pub c1_mission_6: Handle<Image>,
    #[asset(path = "images/missions/recap_guild_scroll.png")]
    pub recap_guild_scroll: Handle<Image>,
}

impl MyAssets {
    pub fn get_item_atlas_path(&self, item: &ItemEnum) -> Handle<Image> {
        return match item {
            ItemEnum::Weapon(_) => self.weapons_atlas.clone(),
            ItemEnum::Armor(_) => self.armors_atlas.clone(),
            ItemEnum::Scroll(_, _) => self.scrolls_atlas.clone(),
        };
    }

    pub fn get_item_loot_atlas_path(&self, item: &ItemLootEnum) -> Handle<Image> {
        return match item {
            ItemLootEnum::Weapon(_) => self.weapons_atlas.clone(),
            ItemLootEnum::Armor(_) => self.armors_atlas.clone(),
            ItemLootEnum::Scroll(_) => self.scrolls_atlas.clone(),
        };
    }

    pub fn get_map_type_atlas(&self) -> Handle<Image> {
        return self.map_type_atlas.clone();
    }

    pub fn load_sound(&self, sound_enum: SoundEnum) -> Handle<AudioSource> {
        return match sound_enum {
            SoundEnum::BookThrowDown => self.book_throw_down.clone(),
            SoundEnum::CockrelMorning => self.cockrel_morning.clone(),
            SoundEnum::EquipArmor => self.equip_armor.clone(),
            SoundEnum::EquipmentEquip => self.equipment_equip.clone(),
            SoundEnum::EquipScroll => self.equip_scroll.clone(),
            SoundEnum::EquipWeapon => self.equip_weapon.clone(),
            SoundEnum::KeysRemovedFromDoor => self.keys_removed_from_door.clone(),
            SoundEnum::PaperTouch => self.paper_touch.clone(),
            SoundEnum::PencilSign => self.pencil_sign.clone(),
            SoundEnum::PickingGolds => self.picking_golds.clone(),
            SoundEnum::SimpleHolidaysV3 => self.simple_holidays_v3.clone(),
        };
    }

    pub fn get_image_map(&self, map_enum: MapImageEnum) -> Handle<Image> {
        return match map_enum {
            MapImageEnum::CampagnTuto => self.map_tuto.clone(),
        };
    }

    pub fn get_mission_image(&self, mission_id: u16) -> Handle<Image> {
        return match mission_id {
            0 => self.c1_mission_1.clone(),
            1 => self.c1_mission_2.clone(),
            2 => self.c1_mission_3.clone(),
            3 => self.c1_mission_4.clone(),
            4 => self.c1_mission_5.clone(),
            5 => self.c1_mission_6.clone(),
            6 => self.c1_mission_6.clone(),
            _ => self.c1_mission_1.clone(),
        };
    }
}