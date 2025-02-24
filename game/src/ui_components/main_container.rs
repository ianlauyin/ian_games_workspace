use bevy::prelude::*;

use crate::constant::ZIndex;

#[derive(Component)]
pub struct MainContainer;

pub struct MainContainerPlugin;

impl Plugin for MainContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_main_container_on_add);
    }
}

fn handle_main_container_on_add(
    ev: Trigger<OnAdd, MainContainer>,
    mut commands: Commands,
    text_query: Query<(Entity, &Parent), With<Text>>,
) {
    commands.entity(ev.entity()).insert((
        Node {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            padding: UiRect::all(Val::Px(10.)),
            row_gap: Val::Px(5.),
            ..default()
        },
        BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.5)),
        ZIndex::MAINCONTAINER.component(),
    ));
    for (entity, parent) in text_query.iter() {
        if parent.get() == ev.entity() {
            commands.entity(entity).insert(ZIndex::TEXT.component());
        }
    }
}
