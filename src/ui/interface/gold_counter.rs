#![allow(dead_code)]
use crate::{
    enums::{MapImageEnum, SoundEnum},
    structs::{
        equipments::ItemEnum, general_structs::UniqueId, missions::ItemLootEnum,
        player_stats::PlayerStats, trigger_structs::GoldCountTrigger,
    },
    systems::systems_constants::NORMAL_BUTTON,
    ui::{styles::containers_styles::basic_button_style, ui_constants::WOOD_COLOR},
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
    #[asset(path = "images/rooms/office/recap_guild_scroll.png")]
    pub recap_guild_scroll: Handle<Image>,
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
            1 => self.c1_mission_1.clone(),
            2 => self.c1_mission_2.clone(),
            3 => self.c1_mission_3.clone(),
            4 => self.c1_mission_4.clone(),
            5 => self.c1_mission_5.clone(),
            6 => self.c1_mission_6.clone(),
            _ => self.c1_mission_1.clone(),
        };
    }
}

// get_path

pub fn gold_counter(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    // image_assets: Res<MyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(500), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn(NodeBundle {
            style: basic_button_style(),
            background_color: BackgroundColor(WOOD_COLOR),
            ..default()
        })
        .insert(Name::new("Gold counter"))
        // Gold icon
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: my_assets.gold.clone().into(),
                style: Style {
                    // The position absolute make the gold counter visible (z-index)
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            });
        })
        // ! WIP - The only sprite bundle that works
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn((
                SpriteBundle {
                    texture: my_assets.buttons_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(100.0, 0.0, 0.0),
                        scale: Vec3::splat(0.2),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 15,
                },
            ));
        })
        .insert(Name::new("Gold icon"))
        // Gold counter text
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(TextBundle {
                    text: Text::from_section(
                        format! {"Guild level : {} | {gold_counter}", player_stats.guild_level, gold_counter = player_stats.golds},
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(GoldCountTrigger)
                .insert(Name::new("Gold counter text"));
        })
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                .insert(Name::new("Hire recruit button"))
                .insert(UniqueId("waz".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "Buy",
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}
