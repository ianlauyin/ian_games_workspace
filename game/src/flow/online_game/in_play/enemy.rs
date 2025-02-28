use bevy::prelude::*;
use shooting_game_shared::util::EdgeUtil;

use crate::{
    components::{Velocity, UFO},
    states::OnlineGameState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_horizontal_movement.run_if(in_state(OnlineGameState::InPlay)),
        );
    }
}

fn handle_horizontal_movement(mut ufo_query: Query<(&mut Velocity, &Transform), With<UFO>>) {
    let edge = EdgeUtil::ufo();
    for (mut velocity, transform) in ufo_query.iter_mut() {
        let x = transform.translation.x;
        if edge.over_left_in(x) || edge.over_right_in(x) {
            velocity.x = -velocity.x;
        }
    }
}
