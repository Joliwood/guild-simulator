use crate::{
    structs::recruits::SelectedRecruitForEquipment,
    ui::rooms::barrack::recruit_overview_folder::recruit_frame::{
        RecruitFrameTextTrigger, RecruitFrameTrigger,
    },
};
use bevy::prelude::*;

pub fn update_selected_recruit(
    selected_recruit_for_equipment: Res<SelectedRecruitForEquipment>,
    mut query: Query<(&mut ImageNode, &mut RecruitFrameTrigger)>,
    text_query: Single<Entity, (With<RecruitFrameTextTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if let Some(recruit) = &selected_recruit_for_equipment.0 {
        *writer.text(*text_query, 0) = recruit.name.to_string();

        for (mut image_node, _) in query.iter_mut() {
            if selected_recruit_for_equipment.0.is_some() {
                if let Some(texture_atlas) = &mut image_node.texture_atlas {
                    texture_atlas.index = recruit.image_atlas_index.into();
                }
            }
        }
    } else {
        *writer.text(*text_query, 0) = "".to_string();

        for (mut image_node, _) in query.iter_mut() {
            if let Some(texture_atlas) = &mut image_node.texture_atlas {
                texture_atlas.index = 4;
            }
        }
    }
}
