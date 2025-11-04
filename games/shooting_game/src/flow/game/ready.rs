use bevy::prelude::*;

use crate::components::{Health, Player, Score, Spaceship, Velocity};
use crate::res::PlayerTag;
use crate::states::GameState;
use crate::util::EdgeUtil;

pub struct ReadyPlugin;

impl Plugin for ReadyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Ready),
            (spawn_spaceship, setup_score_and_health),
        )
        .add_systems(
            Update,
            check_spaceship_position.run_if(in_state(GameState::Ready)),
        );
    }
}

fn setup_score_and_health(mut commands: Commands, player_tag: Res<PlayerTag>) {
    commands.spawn((Score::new(), Player::new_from_res(&player_tag)));
    commands.spawn((Health::new(), Player::new_from_res(&player_tag)));
}

fn spawn_spaceship(mut commands: Commands, player_tag: Res<PlayerTag>) {
    let edge = EdgeUtil::spaceship();
    commands.spawn((
        Player::new_from_res(&player_tag),
        Spaceship::new(Vec2::new(0., edge.bottom_out())),
        Velocity { x: 0., y: 5. },
    ));
}

fn check_spaceship_position(
    mut next_state: ResMut<NextState<GameState>>,
    mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
) {
    let edge = EdgeUtil::spaceship();
    let Ok((transform, mut velocity)) = spaceship_query.single_mut() else {
        panic!("Spaceship not found in check_spaceship_position");
    };
    if !edge.over_bottom_in(transform.translation.y) {
        velocity.y = 0.;
        next_state.set(GameState::InPlay);
    }
}
