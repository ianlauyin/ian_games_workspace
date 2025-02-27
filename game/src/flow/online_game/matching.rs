use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::{
    res::PlayerTag,
    states::OnlineGameState,
    ui_components::{Blink, MainContainer},
    util::cleanup_components,
};

use super::connection::ReceiveMessageEvent;

pub struct MatchingPlugin;

impl Plugin for MatchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OnlineGameState::Matching), setup_matching_notice)
            .add_observer(handle_matching_message)
            .add_systems(
                OnExit(OnlineGameState::Matching),
                cleanup_components::<MatchingNotice>,
            );
    }
}

#[derive(Component)]
struct MatchingNotice;
fn setup_matching_notice(mut commands: Commands) {
    commands.spawn((MainContainer, MatchingNotice)).with_child((
        TextLayout::new_with_justify(JustifyText::Center),
        Text::new("Matching"),
        Blink::new_with_speed(0.01),
    ));
}

fn handle_matching_message(
    ev: Trigger<ReceiveMessageEvent>,
    mut current_player_tag: ResMut<PlayerTag>,
    current_state: ResMut<State<OnlineGameState>>,
    mut next_state: ResMut<NextState<OnlineGameState>>,
) {
    if *current_state.get() != OnlineGameState::Matching {
        return;
    }
    match ev.0 {
        ServerMessage::Joined { player_tag } => current_player_tag.0 = player_tag,
        ServerMessage::GameReady => next_state.set(OnlineGameState::Ready),
        _ => {}
    }
}
