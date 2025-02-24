use bevy::prelude::*;

use crate::components::UFO;

use super::AddScoreEvent;

#[derive(Event)]
pub struct RemoveUFOEvent {
    ufo: Entity,
    by: Option<u8>,
}

impl RemoveUFOEvent {
    pub fn by_player(ufo: Entity, player: u8) -> Self {
        Self {
            ufo,
            by: Some(player),
        }
    }

    pub fn clean_up(ufo: Entity) -> Self {
        Self { ufo, by: None }
    }
}

pub struct RemoveUFOPlugin;

impl Plugin for RemoveUFOPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_remove_ufo);
    }
}

fn handle_remove_ufo(
    ev: Trigger<RemoveUFOEvent>,
    mut commands: Commands,
    ufo_query: Query<Entity, With<UFO>>,
) {
    let ufo = ufo_query.get(ev.ufo).unwrap();
    if let Some(player_tag) = ev.by {
        commands.trigger(AddScoreEvent::new(player_tag, 1));
    }
    if let Some(mut entity_commands) = commands.get_entity(ufo) {
        entity_commands.despawn();
    }
}
