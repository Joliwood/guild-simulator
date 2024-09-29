use crate::{
    enums::RecruitEnum,
    structs::{
        general_structs::{RecruitStats, SelectedRecruit},
        trigger_structs::SelectedRecruitTrigger,
    },
};
use bevy::prelude::*;
use uuid::Uuid;

// #[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
// pub struct RecruitStats {
//     pub class: RecruitEnum,
//     pub endurance: u16,
//     pub experience: u32,
//     pub id: Uuid,
//     pub intelligence: u16,
//     pub level: u8,
//     pub max_experience: u32,
//     pub strength: u16,
// }

pub fn get_selected_recruit(selected_recruit: &Res<SelectedRecruit>) -> RecruitStats {
    match selected_recruit.0 {
        Some(_) => {
            return RecruitStats {
                class: selected_recruit.0.as_ref().unwrap().class.clone(),
                endurance: selected_recruit.0.as_ref().unwrap().endurance,
                experience: selected_recruit.0.as_ref().unwrap().experience,
                id: selected_recruit.0.as_ref().unwrap().id,
                intelligence: selected_recruit.0.as_ref().unwrap().intelligence,
                level: selected_recruit.0.as_ref().unwrap().level,
                max_experience: selected_recruit.0.as_ref().unwrap().max_experience,
                name: selected_recruit.0.as_ref().unwrap().name.clone(),
                strength: selected_recruit.0.as_ref().unwrap().strength,
            };
        }
        None => {
            return RecruitStats {
                class: RecruitEnum::Warrior,
                endurance: 0,
                experience: 0,
                id: Uuid::new_v4(),
                intelligence: 0,
                level: 0,
                max_experience: 0,
                name: "No recruit selected".to_string(),
                strength: 0,
            };
        }
    }
}

pub fn recruit_frame(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
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
    let selected_recruit_data = get_selected_recruit(selected_recruit);

    parent
        .spawn(ImageBundle {
            image: frame_image_handle.into(),
            style: Style {
                // position_type: PositionType::Absolute,
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
                    format!("{}", selected_recruit_data.name),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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
                    image: recruit_handle_frame.into(),
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
                    index: 0,
                    layout: recruit_texture_atlas_layout.clone(),
                },
            ));
        });
}
