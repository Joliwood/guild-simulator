use crate::{structs::maps::Map, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

// External function for the center area (1 big child)
pub fn map_on_table(parent: &mut ChildBuilder, my_assets: &Res<MyAssets>, map: &Option<Map>) {
    parent
        .spawn(ImageBundle {
            image: my_assets.inventory_container.clone().into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            ..default()
        })
        .with_children(|button| {
            if map.is_some() {
                button.spawn(ImageBundle {
                    image: my_assets.get_image_map(map.clone().unwrap().image).into(),
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                });
                // .with_children(|map| {
                //     map.spawn(TextBundle {
                //         text: Text::from_section(
                //             "Center Area Content",
                //             TextStyle {
                //                 font: my_assets.fira_sans_bold.clone(),
                //                 font_size: 32.0,
                //                 color: Color::WHITE,
                //             },
                //         ),
                //         ..default()
                //     });
                // });
            }
        });
}
