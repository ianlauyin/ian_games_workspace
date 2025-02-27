use bevy::prelude::*;
use shooting_game_shared::ClientMessage;

use crate::{
    components::{Bullet, SelfPlayer, Spaceship},
    flow::online_game::connection::SendMessageEvent,
    states::OnlineGameState,
};

pub struct NoticePlayerInfoPlugin;

impl Plugin for NoticePlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            notice_player_info
                .run_if(in_state(OnlineGameState::Ready).or(in_state(OnlineGameState::InPlay))),
        );
    }
}

fn notice_player_info(
    mut commands: Commands,
    spaceship_q: Query<&Spaceship, With<SelfPlayer>>,
    bullet_q: Query<&Bullet, With<SelfPlayer>>,
) {
    let Ok(spaceship) = spaceship_q.get_single() else {
        warn!("Should only have one spaceship with SelfPlayer in notice_player_info");
        return;
    };

    let bullets = bullet_q
        .iter()
        .map(|bullet| bullet.get_position_tuple())
        .collect();

    commands.trigger(SendMessageEvent(ClientMessage::UpdatePlayerInfo {
        position: spaceship.get_position_tuple(),
        bullets,
    }));
}
