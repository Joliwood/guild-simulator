use bevy::prelude::*;

use crate::{
    my_assets::MyAssets,
    structs::daily_events::{DailyEvent, SpontaneousApplication},
};

pub fn spontaneous_application_event_doc(
    commands: &mut Commands,
    my_assets: &Res<MyAssets>,
    spontaneous_application: SpontaneousApplication,
    // texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(ImageBundle {
        image: my_assets.daily_event_document.clone().into(),
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    });
}
