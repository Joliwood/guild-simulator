use crate::structs::{general_structs::SelectedRecruit, trigger_structs::SelectedRecruitTrigger};
use bevy::prelude::*;

pub fn recruit_frame(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_image_handle: Handle<Image> =
        asset_server.load("images/rooms/barrack/recruit_frame.png");
    let recruit_handle_frame: Handle<Image> =
        asset_server.load("images/recruits/recruit_picture_atlas.png");

    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn(ImageBundle {
            image: frame_image_handle.into(),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(60.0),
                ..default()
            },
            ..default()
        })
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
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn((
                ImageBundle {
                    image: recruit_handle_frame.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(60.0),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: 1,
                    layout: recruit_texture_atlas_layout.clone(),
                },
            ));
        });
}
