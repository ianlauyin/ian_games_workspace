use bevy::prelude::*;
use shooting_game_shared::ClientMessage;

use crate::{
    components::{Bullet, SelfPlayer, Spaceship},
    flow::online_game::connection::SendMessageEvent,
    states::OnlineGameState,
};

pub struct SendPlayerInfoPlugin;

impl Plugin for SendPlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_player_info
                .run_if(in_state(OnlineGameState::Ready).or(in_state(OnlineGameState::InPlay))),
        );
    }
}

fn send_player_info(
    mut commands: Commands,
    spaceship_q: Query<&Spaceship, With<SelfPlayer>>,
    bullet_q: Query<&Bullet, With<SelfPlayer>>,
) {
    let position = spaceship_q
        .single()
        .map(|spaceship| Some(spaceship.get_position_tuple()))
        .unwrap_or(None);

    let bullets = bullet_q
        .iter()
        .map(|bullet| bullet.get_position_tuple())
        .collect();

    commands.trigger(SendMessageEvent(ClientMessage::UpdatePlayerInfo {
        position,
        bullets,
    }));
}
