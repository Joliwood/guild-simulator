use crate::{
    // audio::play_sound::play_sound,
    enums::ColorPaletteEnum,
    structs::{
        general_structs::MissionNotificationsNumber,
        trigger_structs::{MissionNotificationTrigger, NotificationToastTrigger},
    },
    utils::get_mission_notification_tooltip_text,
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn spawn_or_update_notification(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let texture_handle: Handle<Image> = asset_server.load("images/ui/notification_atlas.png");
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
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // play_sound(&asset_server, &mut commands, SoundEnum::PaperTouch);

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
            .insert(NotificationToastTrigger)
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Px(60.),
                                height: Val::Px(60.),
                                // margin: UiRect::all(Val::Px(5.)),
                                padding: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::ZERO,
                                    top: Val::ZERO,
                                    bottom: Val::ZERO,
                                },
                                ..default()
                            },
                            // image: texture_handle.clone().into(),
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
                        Tooltip::cursor(get_mission_notification_tooltip_text(
                            mission_notifications_number.0 + 1,
                        ))
                        .with_activation(TooltipActivation::IMMEDIATE), // Display the current number in the tooltip
                    ))
                    .insert(MissionNotificationTrigger)
                    .with_children(|parent| {
                        parent.spawn((TextBundle::from_section(
                            format!("x{}", mission_notifications_number.0 + 1),
                            TextStyle {
                                font: asset_server.clone().load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: ColorPaletteEnum::DarkBrown.as_color(),
                            },
                        ),));
                    });
            });

        info!(
            "Spawned new toast with number: {}",
            mission_notifications_number.0
        );

        // Increment the mission notification number for the next toast
        mission_notifications_number.0 += 1;
    }
}
