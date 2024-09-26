// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod custom_components;
mod enums;
mod structs;
mod systems;
mod ui;
mod utils;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyri_tooltip::prelude::*;
use structs::general_structs::{
    MissionModalVisible, Missions, PlayerStats, SelectedMission, SelectedRecruit, ToastQueue,
};
use ui::interface::gold_counter::MyAssets;
// use structs::{MissionModalVisible, Missions, PlayerStats, SelectedMission, SelectedRecruit};
use bevy_ui_mod_alerts::AlertsPlugin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct MySystems;

#[derive(Component)]
pub struct AlertButton;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Desactivate on testing
            WorldInspectorPlugin::new(),
            TooltipPlugin::default(),
            AlertsPlugin::new(),
        ))
        .insert_resource(PlayerStats::default())
        .insert_resource(Missions::default())
        .insert_resource(SelectedRecruit::default())
        .insert_resource(SelectedMission::default())
        .insert_resource(MissionModalVisible(false))
        .insert_resource(ToastQueue {
            toasts: VecDeque::new(),
            max_toasts: 3,
        })
        .init_collection::<MyAssets>()
        .add_systems(
            Startup,
            (
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                systems::inputs::mouse_systems::mouse_init,
                ui::buttons::room_arrows::room_bottom_arrow_button,
                ui::buttons::room_arrows::room_left_arrow_button,
                ui::buttons::room_arrows::room_right_arrow_button,
                ui::buttons::room_arrows::room_top_arrow_button,
                ui::interface::gold_counter::gold_counter,
                ui::interface::room_interface_text::room_interface_text,
                ui::rooms::room_setup::room_setup,
                systems::recruits::hiring_setup::hiring_setup,
                // ui::interface::toasts::notification_toast::notification_toast,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::update_buttons::move_room_from_keyboard,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::update_gold_counter::update_gold_counter,
                systems::updates::update_room_interface_text::update_room_interface_text,
                systems::updates::update_room::update_room,
                systems::updates::update_buttons::mouse_interaction_updates,
                systems::updates::update_buttons::buttons_disable_updates,
                systems::updates::update_buttons::select_recruit_button,
                systems::updates::update_buttons::select_mission_button,
                systems::updates::update_buttons::assign_recruit_to_mission,
                systems::updates::update_buttons::close_mission_modal,
                systems::updates::update_buttons::start_mission_button,
                systems::updates::update_buttons::select_item_in_inventory,
                systems::updates::update_recruit_infos::update_recruit_infos,
                systems::updates::update_selected_recruit::update_selected_mission_recruit_id,
                systems::updates::update_selected_recruit::update_update_selected_mission_percentage_of_victory,
                ui::modals::mission_details_modal::display_mission_modal,
                handle_toast_interaction,
                spawn_toast_on_keypress,
            ),
        )
        .run()
}

// Struct to represent a Toast Message
#[derive(Component)]
struct Toast;

fn handle_toast_interaction(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Toast>)>,
    mut toast_queue: ResMut<ToastQueue>,
    mut query: Query<&mut Style, With<Toast>>,
) {
    // Handle interaction: when a toast is clicked, remove it
    for (entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            info!("Toast clicked");
            // Remove the toast from the queue
            toast_queue.toasts.retain(|&e| e != entity);
            commands.entity(entity).despawn_recursive();

            // Update positions of remaining toasts
            for (i, &toast) in toast_queue.toasts.iter().enumerate() {
                if let Ok(mut style) = query.get_mut(toast) {
                    style.bottom = Val::Px(10.0 * (i as f32 + 1.0));
                }
            }
        }
    }
}

fn spawn_toast_on_keypress(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut toast_queue: ResMut<ToastQueue>,
    mut query: Query<&mut Style, With<Toast>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/ui/notification_atlas.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 50),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    if keyboard_input.just_pressed(KeyCode::KeyF) {
        // If there are already 3 toasts, remove the oldest one
        if toast_queue.toasts.len() >= toast_queue.max_toasts {
            if let Some(oldest_toast) = toast_queue.toasts.pop_front() {
                commands.entity(oldest_toast).despawn_recursive();
            }
        }

        // Spawn a new toast notification
        let toast = commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    right: Val::Px(0.),
                    top: Val::Px(70.0 * (toast_queue.toasts.len() as f32) + 50.), // Position toasts with 10px gap
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(60.),
                                height: Val::Px(60.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            image: texture_handle.clone().into(),
                            border_radius: BorderRadius {
                                top_left: Val::Px(10.),
                                top_right: Val::ZERO,
                                bottom_left: Val::Px(10.),
                                bottom_right: Val::ZERO,
                            },
                            ..default()
                        },
                        TextureAtlas {
                            index: 0,
                            layout: texture_atlas_layout.clone(),
                        },
                        Tooltip::cursor("Mission finished !\n\n Go check out your rapports")
                            .with_activation(TooltipActivation::IMMEDIATE),
                    ))
                    .insert(Toast);
            })
            .id();

        // Add the new toast to the queue
        toast_queue.toasts.push_back(toast);

        // Update positions of all toasts
        for (i, &toast) in toast_queue.toasts.iter().enumerate() {
            if let Ok(mut style) = query.get_mut(toast) {
                style.top = Val::Px(70.0 * (i as f32) + 50.);
            }
        }
    }
}
