use crate::ui::interface::gold_counter::MyAssets;
use bevy::prelude::*;

pub fn recap_guild_scroll(my_assets: &Res<MyAssets>, elements_on_desk: &mut ChildBuilder) {
    elements_on_desk.spawn(ImageBundle {
        image: my_assets.recap_guild_scroll.clone().into(),
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            width: Val::Px(550. / 2.),
            height: Val::Px(140. / 2.),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
