use crate::structs::{general_structs::SelectedRecruit, trigger_structs::SelectedRecruitTrigger};
use bevy::prelude::*;

pub fn recruit_infos(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_image_handle: Handle<Image> =
        asset_server.load("images/rooms/barrack/recruit_infos.png");

    parent
        .spawn(ImageBundle {
            image: frame_image_handle.into(),
            style: Style {
                // position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                ..default()
            },
            z_index: ZIndex::Global(2),
            ..default()
        })
        .insert(Name::new("Barrack > recruit overview > recruit infos"))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Recruit name",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(20.0),
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..Default::default()
                },
                z_index: ZIndex::Global(3),
                ..Default::default()
            });
        });
}
