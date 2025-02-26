use bevy::prelude::*;
use shooting_game_shared::util::EdgeUtil;

use crate::components::{Health, Player, Score, Spaceship, Velocity};
use crate::{constant::SPACESHIP_SIZE, states::GameState};

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

fn setup_score_and_health(mut commands: Commands) {
    commands.spawn((Score::new(), Player(1)));
    commands.spawn((Health::new(), Player(1)));
}

fn spawn_spaceship(mut commands: Commands) {
    let edge = EdgeUtil::new(SPACESHIP_SIZE);
    commands.spawn((
        Player(1),
        Spaceship::new(Vec2::new(0., edge.bottom_out())),
        Velocity { x: 0., y: 5. },
    ));
}

fn check_spaceship_position(
    mut next_state: ResMut<NextState<GameState>>,
    mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
) {
    let edge = EdgeUtil::new(SPACESHIP_SIZE);
    let Ok((transform, mut velocity)) = spaceship_query.get_single_mut() else {
        panic!("Spaceship not found in check_spaceship_position");
    };
    if !edge.over_bottom_in(transform.translation.y) {
        velocity.y = 0.;
        next_state.set(GameState::InPlay);
    }
}
