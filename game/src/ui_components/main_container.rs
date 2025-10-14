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
    ev: On<Add, MainContainer>,
    mut commands: Commands,
    text_query: Query<(Entity, &ChildOf), With<Text>>,
) {
    if let Ok(mut entity_commands) = commands.get_entity(ev.entity) {
        entity_commands.insert((
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
    }
    for (entity, parent) in text_query.iter() {
        if parent.parent() == ev.entity {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(ZIndex::TEXT.component());
            }
        }
    }
}
