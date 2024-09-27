// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod custom_components;
mod enums;
mod structs;
mod systems;
mod ui;
mod utils;

use bevy::{prelude::*, reflect::NamedField};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyri_tooltip::prelude::*;
use structs::general_structs::{
    MissionModalVisible, MissionNotificationsNumber, Missions, PlayerStats, SelectedMission,
    SelectedRecruit,
};
use ui::interface::gold_counter::MyAssets;
// use structs::{MissionModalVisible, Missions, PlayerStats, SelectedMission, SelectedRecruit};

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
            // AlertsPlugin::new(),
        ))
        .insert_resource(PlayerStats::default())
        .insert_resource(Missions::default())
        .insert_resource(SelectedRecruit::default())
        .insert_resource(SelectedMission::default())
        .insert_resource(MissionModalVisible(false))
        .insert_resource(MissionNotificationsNumber(0))
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
                spawn_or_update_toast,
                delete_toast,
            ),
        )
        .run()
}
#[derive(Component)]
struct MissionNotificationTrigger;

#[derive(Component)]
struct NotificationToast; // Component to identify the toast notification entity

// This function handles the interaction (click) on the notification toast
fn handle_toast_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<MissionNotificationTrigger>),
    >,
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
) {
    // If the button is pressed, remove the toast and display the notification number
    for (entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Remove the toast
            // commands.entity(entity).despawn_recursive();

            // Increment the notification number when clicked
            mission_notifications_number.0 = 0;
        }
    }
}

fn delete_toast(
    mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut commands: Commands,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<MissionNotificationTrigger>),
    >,
) {
    for (_entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if mission_notifications_number.is_changed() {
                for entity in query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

#[derive(Debug, Component)]
struct NotificationToastTrigger;

fn spawn_or_update_toast(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // Query for existing notification toast
    // toast_query: Query<(&NotificationToast, &Text, Entity)>,
    // all_toasts_query: NotificationToastTrigger,
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
        // We reset the node before to spawn a new one
        // delete_toast(&mut commands, all_toasts_query);

        // if mission_notifications_number.0 == 0 {
        // If no toast exists, create a new one
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    right: Val::Px(0.),
                    top: Val::Px(120.),
                    ..default()
                },
                ..default()
            })
            .insert((
                Name::new("TO DELETE BROO"),
                NotificationToast,
                NotificationToastTrigger,
            )) // Mark this entity as a toast notification
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
                        Tooltip::cursor(format!(
                            "Mission finished! Notifications: {}",
                            mission_notifications_number.0
                        ))
                        .with_activation(TooltipActivation::IMMEDIATE), // Display the current number in the tooltip
                    ))
                    .insert(MissionNotificationTrigger)
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                format!("x {}", mission_notifications_number.0),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                },
                            ),
                            NotificationToast, // Make sure the `Text` belongs to the toast
                        ));
                    });
            });

        info!(
            "Spawned new toast with number: {}",
            mission_notifications_number.0
        );
        // } else {
        //     // If a toast already exists, update the existing `Text`
        //     for (_toast, mut text) in toast_query.iter_mut() {
        //         // Update the content of the `Text` component
        //         text.sections[0].value = format!("x {}", mission_notifications_number.0);
        //     }
        // }

        // Increment the mission notification number for the next toast
        mission_notifications_number.0 += 1;
    }
}
