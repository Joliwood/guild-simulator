#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]

mod audio;
mod content;
mod custom_components;
mod enums;
// mod locales;
mod my_assets;
mod structs;
mod systems;
mod ui;
mod utils;

// ! Stand-by
// use my_assets::Locales;
// use bevy_asset_loader::asset_collection::AssetCollectionApp;
// use my_assets::{MyAssets, MyAssetsLoader};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyri_tooltip::prelude::*;
// use crate::locales::{en, fr};

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowTheme};
use content::constants::MAX_GAME_SECONDS;
use structs::{
    daily_events_folder::daily_events::{DailyEventTargets, DailyEvents},
    general_structs::{
        DailyEventsModalVisible, DayTime, MissionModalVisible, MissionNotificationsNumber,
        MissionReportsModalVisible, NotificationCount,
    },
    maps::{Maps, SelectedMapId},
    missions::{MissionReports, Missions, SelectedMission},
    player_stats::PlayerStats,
    recruits::{SelectedRecruitForEquipment, SelectedRecruitForMission},
    trigger_structs::{
        BarrackRoomNotificationContainerTrigger, BarrackRoomNotificationTrigger,
        CommandRoomNotificationContainerTrigger, CommandRoomNotificationTrigger,
        OfficeRoomNotificationContainerTrigger, OfficeRoomNotificationTrigger, PlayerDayTrigger,
        RealTimeDayProgressBarTrigger,
    },
};
use systems::updates::skip_tuto::skip_tuto;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            // DefaultPlugins
            DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Guild simulator".into(),
                    resizable: true,
                    window_theme: Some(WindowTheme::Dark),
                    canvas: Some("#mygame-canvas".into()),
                    ..default()
                }),
                ..default()
            }),
            // WorldInspectorPlugin::new(),
            TooltipPlugin::default(),
        ))
        // .init_asset::<MyAssets>()
        // .init_collection::<MyAssets>()
        .insert_resource(PlayerStats::default())
        .insert_resource(MissionReports::default())
        .insert_resource(Missions::default())
        .insert_resource(SelectedRecruitForEquipment::default())
        .insert_resource(SelectedRecruitForMission::default())
        .insert_resource(SelectedMission::default())
        .insert_resource(SelectedMapId::default())
        .insert_resource(MissionModalVisible(false))
        .insert_resource(MissionReportsModalVisible(false))
        .insert_resource(DailyEventsModalVisible(false))
        .insert_resource(MissionNotificationsNumber(0))
        .insert_resource(Maps::default())
        .insert_resource(DailyEvents::default())
        .insert_resource(DailyEventTargets::default())
        .insert_resource(DayTime::default())
        .insert_resource(NotificationCount::default())
        .add_systems(
            Startup,
            (
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                ui::hud_folder::hud::hud,
                setup_i18n,
            ),
        )
        .add_systems(
            Update,
            (
                systems::inputs::escape::close_modal_on_escape,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::barrack::select_recruit_for_equipment_button::select_recruit_for_equipment_button,
                systems::updates::command_room::select_map::select_map,
                systems::updates::command_room::select_mission_button::select_mission_button,
                systems::updates::command_room::select_recruit_for_mission_button::select_recruit_for_mission_button,
                systems::updates::hud::update_day_counter::update_day_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_gold_counter::update_gold_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_guild_level::update_guild_level.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_recruit_counter::update_recruit_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_reputation_counter::update_reputation_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_toxicity_counter::update_toxicity_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::input::move_room_from_keyboard,
                systems::updates::update_room::update_room,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::barrack::select_item_in_inventory::select_item_in_inventory,
                systems::updates::command_room::close_mission_modal::close_mission_modal,
                systems::updates::command_room::start_mission_button::start_mission_button,
                systems::updates::hud::delete_notifications_on_click::delete_notifications_on_click,
                systems::updates::hud::sleep_button_system::sleep_button_system,
                systems::updates::office::select_discussion_answer::select_discussion_answer,
                systems::updates::office::sign_mission_report::sign_mission_report,
                systems::updates::office::toggle_daily_event_documents::toggle_daily_event_documents,
                systems::updates::office::toggle_mission_reports::toggle_mission_reports,
                systems::updates::scroll::scroll,
                systems::updates::update_room_on_click::update_room_on_click,
                ui::modals::daily_events::daily_events_modal::daily_events_modal,
                ui::modals::mission_order_modal_folder::mission_order_modal::mission_order_modal,
                ui::modals::mission_report_modal_folder::mission_report_modal::mission_report_modal,
                update_daytime,
                update_progress_bar,
                systems::updates::hud::update_sleep_button_texture::update_sleep_button_texture,
                skip_tuto,
            ),
        )
        .add_systems(
            Update,
            (
                update_notification_indicators_text_for_command_room.run_if(resource_changed::<NotificationCount>),
                update_notification_indicators_text_for_office_room.run_if(resource_changed::<NotificationCount>),
                update_notification_indicators_text_for_barrack_room.run_if(resource_changed::<NotificationCount>),
            ),
        )
        .run()
}

fn update_daytime(
    mut day_time: ResMut<DayTime>,
    time: Res<Time>,
    _query: Single<Entity, (With<PlayerDayTrigger>, With<Text>)>,
    _writer: TextUiWriter,
) {
    // Si le temps maximal est atteint, on arrête d'incrémenter
    if day_time.second_count >= MAX_GAME_SECONDS {
        return;
    }

    // On ajoute le temps écoulé en secondes au compteur de temps
    day_time.current_time += time.delta_secs();
    day_time.elapsed_time += time.delta_secs();

    // Exécute tous les secondes
    if day_time.elapsed_time >= 1.0 {
        day_time.second_count += 1;
        day_time.elapsed_time -= 1.0;
    }
}

fn update_progress_bar(
    day_time: Res<DayTime>,
    mut query: Query<&mut Node, With<RealTimeDayProgressBarTrigger>>,
) {
    // Calcul du ratio de progression (entre 0.0 et 1.0)
    let progress_ratio = day_time.current_time / MAX_GAME_SECONDS as f32;

    // Mise à jour de la largeur pour chaque entité avec le trigger `RealTimeDayProgressBarTrigger`
    for mut node in query.iter_mut() {
        node.width = Val::Px(progress_ratio * 70.);
    }
}

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
//
i18n!("assets/locales", fallback = "en");

fn setup_i18n() {
    rust_i18n::set_locale("fr");
}

pub fn update_notification_indicators_text_for_command_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<CommandRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &CommandRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.command_room_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.command_room_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_notification_indicators_text_for_office_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<OfficeRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &OfficeRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.office_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.office_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_notification_indicators_text_for_barrack_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<BarrackRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &BarrackRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.barrack_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.barrack_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}
