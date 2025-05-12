use bevy::prelude::*;

use crate::res::PlayerTag;

#[derive(Component, Clone)]
pub struct Player(pub u8);

impl Player {
    pub fn new_from_res(player_tag: &Res<PlayerTag>) -> Self {
        Self(player_tag.0)
    }
}

#[derive(Component)]
pub struct SelfPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_added);
    }
}

fn on_player_added(
    ev: Trigger<OnAdd, Player>,
    mut commands: Commands,
    player_tag: Res<PlayerTag>,
    player_q: Query<&Player>,
) {
    let Ok(player) = player_q.get(ev.target()) else {
        warn!("Player not found in on_player_added");
        return;
    };
    if player.0 == player_tag.0 {
        commands.entity(ev.target()).insert(SelfPlayer);
    }
}
