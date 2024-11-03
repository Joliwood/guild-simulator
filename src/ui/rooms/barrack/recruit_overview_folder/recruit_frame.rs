use crate::{
    my_assets::MyAssets, structs::recruits::SelectedRecruitForEquipment,
    utils::get_selected_recruit_for_equipment,
};
use bevy::prelude::*;

pub fn recruit_frame(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1400, 400),
        7,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);
    let selected_recruit_for_equipment_data =
        get_selected_recruit_for_equipment(selected_recruit_for_equipment);

    parent
        .spawn((
            UiImage {
                image: my_assets.recruit_frame.clone().into(),
                ..default()
            },
            Node {
                width: Val::Px(180.),
                height: Val::Px(330.),
                ..default()
            },
            ZIndex(2),
        ))
        .insert(Name::new("Barrack > recruit overview > recruit frame"))
        .with_children(|parent| {
            parent.spawn((
                Text::new(selected_recruit_for_equipment_data.name.to_string()),
                TextFont {
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(20.0),
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..Default::default()
                },
                ZIndex(3),
            ));

            parent.spawn((
                UiImage::from_atlas_image(
                    my_assets.recruit_picture_atlas.clone().into(),
                    TextureAtlas {
                        index: selected_recruit_for_equipment_data.image_atlas_index.into(),
                        layout: recruit_texture_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ZIndex(1),
            ));
        });
}
