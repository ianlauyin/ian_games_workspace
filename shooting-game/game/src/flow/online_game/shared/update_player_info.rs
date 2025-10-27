use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::{
    flow::online_game::{connection::ReceiveMessageEvent, trigger::UpdatePositionEvent},
    res::PlayerTag,
    states::OnlineGameState,
};

pub struct UpdatePlayerInfoPlugin;

impl Plugin for UpdatePlayerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_player_info);
    }
}

fn update_player_info(
    trigger: On<ReceiveMessageEvent>,
    mut commands: Commands,
    current_state: ResMut<State<OnlineGameState>>,
    self_player_tag: Res<PlayerTag>,
) {
    match current_state.get() {
        OnlineGameState::Ready | OnlineGameState::InPlay => {}
        _ => return,
    }

    match trigger.event().0.clone() {
        ServerMessage::UpdatePosition {
            player_tag,
            position,
            bullets,
        } => {
            if player_tag != self_player_tag.0 {
                commands.trigger(UpdatePositionEvent {
                    player_tag,
                    position: Vec2::new(position.0, position.1),
                    bullets,
                });
            }
        }
        _ => {}
    }
}
