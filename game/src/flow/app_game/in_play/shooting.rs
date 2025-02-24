use bevy::prelude::*;

use crate::{
    components::{Bullet, Player, Spaceship},
    res::{ControlMode, ControlOption},
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shooting_bullet);
    }
}

fn shooting_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    control_option: Res<ControlOption>,
    mut spaceship_query: Query<(&mut Spaceship, &Player)>,
) {
    if keys.pressed(KeyCode::Space) || control_option.mode == ControlMode::Button {
        let Ok((mut spaceship, player)) = spaceship_query.get_single_mut() else {
            return;
        };
        if spaceship.can_shoot() {
            commands.spawn(Bullet::by_player(player.0));
            spaceship.start_cd();
        }
    }
}
