use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::{
    flow::online_game::{
        connection::ReceiveMessageEvent,
        trigger::{
            AddScoreEvent, DestroyEnemyEvent, PlayerDamagedEvent, RemoveBulletEvent,
            SpawnEnemyEvent,
        },
    },
    res::PlayerTag,
    states::OnlineGameState,
};

pub struct FromServerPlugin;

impl Plugin for FromServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(listen_from_server);
    }
}

fn listen_from_server(
    ev: Trigger<ReceiveMessageEvent>,
    commands: Commands,
    current_state: Res<State<OnlineGameState>>,
    self_player_tag: Res<PlayerTag>,
    next_state: ResMut<NextState<OnlineGameState>>,
) {
    match ev.0 {
        ServerMessage::SpawnEnemy {
            tag,
            position,
            velocity,
        } => {
            if *current_state.get() == OnlineGameState::InPlay {
                handle_spawn_enemy(commands, tag, position, velocity);
            }
        }
        ServerMessage::ConfirmDamaged {
            player_tag,
            enemy_tag,
            health,
        } => handle_confirm_damaged(commands, player_tag, enemy_tag, health),
        ServerMessage::ConfirmDestroyEnemy {
            player_tag,
            bullet_tag,
            enemy_tag,
            new_score,
        } => handle_confirm_destroy_enemy(
            commands,
            self_player_tag,
            player_tag,
            bullet_tag,
            enemy_tag,
            new_score,
        ),
        ServerMessage::GameOver => handle_game_over(next_state),
        _ => {}
    }
}

fn handle_spawn_enemy(
    mut commands: Commands,
    tag: u16,
    tuple_position: (f32, f32),
    tuple_velocity: (f32, f32),
) {
    let position = Vec2::new(tuple_position.0, tuple_position.1);
    let velocity = Vec2::new(tuple_velocity.0, tuple_velocity.1);
    commands.trigger(SpawnEnemyEvent {
        tag,
        position,
        velocity,
    });
}

fn handle_confirm_damaged(mut commands: Commands, player_tag: u8, enemy_tag: u16, health: u8) {
    commands.trigger(DestroyEnemyEvent(enemy_tag));
    commands.trigger(PlayerDamagedEvent::update_health(player_tag, health));
}

fn handle_confirm_destroy_enemy(
    mut commands: Commands,
    self_player_tag: Res<PlayerTag>,
    player_tag: u8,
    bullet_tag: u16,
    enemy_tag: u16,
    new_score: u8,
) {
    commands.trigger(DestroyEnemyEvent(enemy_tag));
    commands.trigger(AddScoreEvent {
        player_tag,
        score: new_score,
    });
    if self_player_tag.0 == player_tag {
        commands.trigger(RemoveBulletEvent(bullet_tag));
    }
}

fn handle_game_over(mut next_state: ResMut<NextState<OnlineGameState>>) {
    next_state.set(OnlineGameState::Result);
}
