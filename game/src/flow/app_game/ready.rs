use bevy::prelude::*;

use crate::{
    constant::{MOBILE_WINDOW_SIZE, SPACESHIP_SIZE},
    game_component::{Health, Player, Score, Spaceship},
    states::GameState,
    ui_component::Velocity,
};

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
    commands.spawn((
        Player(1),
        Spaceship::new(Vec2::new(0., -MOBILE_WINDOW_SIZE.y / 2. - SPACESHIP_SIZE.y)),
        Velocity { x: 0., y: 5. },
    ));
}

fn check_spaceship_position(
    mut next_state: ResMut<NextState<GameState>>,
    mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
) {
    let (transform, mut velocity) = spaceship_query.get_single_mut().unwrap();
    if transform.translation.y >= -MOBILE_WINDOW_SIZE.y / 2. + SPACESHIP_SIZE.y {
        velocity.y = 0.;
        next_state.set(GameState::InPlay);
    }
}
