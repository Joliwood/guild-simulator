use crate::{
    enums::ColorPaletteEnum,
    structs::{maps::Map, missions::SelectedMission},
    ui::interface::gold_counter::MyAssets,
};
use bevy::prelude::*;

pub fn start_mission(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    selected_map: &Option<Map>,
    selected_mission: &mut ResMut<SelectedMission>,
) {
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                padding: UiRect {
                    left: Val::Px(30.),
                    right: Val::Px(30.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|column| {
            if selected_mission.mission.is_some() {
                if let Some(map) = selected_map {
                    column.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Chance of win : {}", map.name.clone()),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    // After the existing children have been added
                    column
                        .spawn(ButtonBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(10.0),
                                right: Val::Px(10.0),
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            background_color: BackgroundColor(
                                ColorPaletteEnum::DarkBrown.as_color(),
                            ),
                            ..default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::from_section(
                                    "Start the mission",
                                    TextStyle {
                                        font: my_assets.fira_sans_bold.clone(),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                }
            }
        });
}
