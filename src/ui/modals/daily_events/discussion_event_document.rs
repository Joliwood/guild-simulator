use bevy::prelude::*;

use crate::{my_assets::MyAssets, structs::daily_events::DailyEvent};

pub fn discussion_event_document(
    commands: &mut Commands,
    my_assets: &Res<MyAssets>,
    last_daily_event: &DailyEvent,
    // texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(ImageBundle {
        image: my_assets.mission_notification_document.clone().into(),
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    });
}
