use crate::{
    custom_components::CustomButton,
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
        trigger_structs::{
            SelectedMissionPercentOfVictoryTrigger, SelectedMissionRecruitIdTrigger,
        },
    },
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

// Middle container
pub fn spawn_middle_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_mission: &Res<SelectedMission>,
    _missions: Res<Missions>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(40.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(Color::WHITE),
            ..default()
        })
        .with_children(|parent| {
            // Middle content
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Mission middle \n Assigned recruit :",
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        format!("{:?}", selected_mission.recruit_id),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(SelectedMissionRecruitIdTrigger);

            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        format!("{:?}", selected_mission.percent_of_victory),
                        TextStyle {
                            font: my_assets.fira_sans_bold.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(SelectedMissionPercentOfVictoryTrigger);

            // Button inside the middle container
            parent
                .spawn(CustomButton::Primary.bundle(my_assets))
                .insert(UniqueId("start_mission".to_string()))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Start the mission",
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
        });
}
