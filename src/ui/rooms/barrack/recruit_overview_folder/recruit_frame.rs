use crate::{
    structs::general_structs::SelectedRecruit, ui::interface::gold_counter::MyAssets,
    utils::get_selected_recruit,
};
use bevy::prelude::*;

pub fn recruit_frame(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // let frame_image_handle: Handle<Image> =
    //     my_assets.load("images/rooms/barrack/recruit_frame.png");
    // let recruit_image_handle: Handle<Image> =
    //     my_assets.load("images/recruits/recruit_picture_atlas.png");

    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);
    let selected_recruit_data = get_selected_recruit(selected_recruit);

    parent
        .spawn(ImageBundle {
            image: my_assets.recruit_frame.clone().into(),
            style: Style {
                width: Val::Px(200.),
                height: Val::Px(350.),
                ..default()
            },
            z_index: ZIndex::Global(2),
            ..default()
        })
        .insert(Name::new("Barrack > recruit overview > recruit frame"))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    selected_recruit_data.name.to_string(),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
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

            parent.spawn((
                ImageBundle {
                    image: my_assets.recruit_picture_atlas.clone().into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    z_index: ZIndex::Global(1),
                    ..default()
                },
                TextureAtlas {
                    index: selected_recruit_data.image_atlas_index.into(),
                    layout: recruit_texture_atlas_layout.clone(),
                },
            ));
        });
}
