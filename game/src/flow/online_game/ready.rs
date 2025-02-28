use crate::components::{Health, Player, Score, SelfPlayer, Spaceship, Velocity};
use crate::res::PlayerTag;
use crate::states::OnlineGameState;
use bevy::prelude::*;
use shooting_game_shared::util::EdgeUtil;
use shooting_game_shared::ServerMessage;

use super::connection::ReceiveMessageEvent;

pub struct ReadyPlugin;

impl Plugin for ReadyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(OnlineGameState::Ready),
            (spawn_spaceship, setup_score_and_health),
        )
        .add_systems(OnExit(OnlineGameState::Ready), stop_spaceship)
        .add_observer(listen_message);
    }
}

fn setup_score_and_health(mut commands: Commands) {
    for i in 1..=2 {
        commands.spawn((Score::new(), Player(i)));
        commands.spawn((Health::new(), Player(i)));
    }
}

fn spawn_spaceship(mut commands: Commands, player_tag: Res<PlayerTag>) {
    let edge = EdgeUtil::spaceship();
    for i in 1..=2 {
        let y = if i == player_tag.0 { 5. } else { 0. };
        commands.spawn((
            Player(i),
            Spaceship::new(Vec2::new(
                if i == 1 { -100. } else { 100. },
                edge.bottom_out(),
            )),
            Velocity { x: 0., y },
        ));
    }
}

fn stop_spaceship(mut spaceship_q: Query<&mut Velocity, (With<Spaceship>, With<SelfPlayer>)>) {
    let Ok(mut velocity) = spaceship_q.get_single_mut() else {
        warn!("Should only have one spaceship with SelfPlayer in stop_spaceship");
        return;
    };
    velocity.x = 0.;
    velocity.y = 0.;
}

fn listen_message(
    trigger: Trigger<ReceiveMessageEvent>,
    current_state: ResMut<State<OnlineGameState>>,
    mut next_state: ResMut<NextState<OnlineGameState>>,
) {
    if *current_state.get() != OnlineGameState::Ready {
        return;
    }
    match trigger.event().0.clone() {
        ServerMessage::GameStart => {
            next_state.set(OnlineGameState::InPlay);
        }
        _ => {}
    }
}
