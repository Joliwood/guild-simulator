use crate::ui::interface::gold_counter::MyAssets;
use bevy::prelude::*;

// External function for the center area (1 big child)
pub fn map_on_table(parent: &mut ChildBuilder, my_assets: &Res<MyAssets>) {
    parent
        .spawn(ImageBundle {
            image: my_assets.wood_box_container.clone().into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Percent(30.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    "Center Area Content",
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}
