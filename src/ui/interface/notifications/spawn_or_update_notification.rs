use crate::{
    audio::play_sound::play_sound,
    enums::{ColorPaletteEnum, SoundEnum},
    structs::{
        missions::MissionReports,
        trigger_structs::{MissionNotificationTrigger, NotificationToastTrigger},
    },
    ui::interface::gold_counter::MyAssets,
    utils::get_mission_notification_tooltip_text,
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn spawn_or_update_notification(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    my_assets: Res<MyAssets>,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 50),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mission_notifications_number = mission_reports.0.len();

    play_sound(asset_server, commands, SoundEnum::PaperTouch);

    // Create a new notification node
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(40.),
                height: Val::Px(40.),
                right: Val::Px(0.),
                top: Val::Px(120.),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("---> Notification toast"))
        .insert(NotificationToastTrigger)
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(40.),
                            height: Val::Px(40.),
                            padding: UiRect {
                                left: Val::Px(10.),
                                right: Val::ZERO,
                                top: Val::ZERO,
                                bottom: Val::ZERO,
                            },
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
                    Tooltip::cursor(get_mission_notification_tooltip_text(
                        mission_notifications_number as u8,
                    ))
                    .with_activation(TooltipActivation::IMMEDIATE),
                ))
                .insert(MissionNotificationTrigger)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("x{}", mission_notifications_number),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.,
                            color: ColorPaletteEnum::DarkBrown.as_color(),
                        },
                    ));
                });
        });
}
