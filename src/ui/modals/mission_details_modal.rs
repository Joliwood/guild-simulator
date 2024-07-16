use crate::structs::ModalVisible;
use bevy::{asset::AssetServer, prelude::*};

pub fn display_mission_modal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    modal_visible: Res<ModalVisible>,
    query: Query<Entity, With<ModalVisible>>,
) {
    // Despawn existing modals
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if modal_visible.is_changed() && modal_visible.0 {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Mission Details",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            });
    }
}
