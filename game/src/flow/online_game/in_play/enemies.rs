use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::{
    flow::online_game::{connection::ReceiveMessageEvent, trigger::SpawnEnemyEvent},
    states::OnlineGameState,
};

#[derive(Component)]
pub struct EnemyTag(pub u16);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(trigger_enemy_spawn);
    }
}

fn trigger_enemy_spawn(
    ev: Trigger<ReceiveMessageEvent>,
    mut commands: Commands,
    current_state: Res<State<OnlineGameState>>,
) {
    if *current_state.get() != OnlineGameState::InPlay {
        return;
    }
    match ev.0 {
        ServerMessage::SpawnEnemy {
            tag,
            position,
            velocity,
        } => {
            commands.trigger(SpawnEnemyEvent {
                tag,
                position,
                velocity,
            });
        }
        _ => {}
    }
}
