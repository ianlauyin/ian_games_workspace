use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;

use crate::game::Spaceship;
use crate::states::GameState;

#[derive(Event)]
pub struct InvisibleEvent;

pub struct InvisiblePlugin;

impl Plugin for InvisiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_invisible.run_if(in_state(GameState::InPlay)))
            .add_observer(trigger_invisible_ship);
    }
}

#[derive(Component)]
pub struct Invisible {
    timer: Timer,
}

fn trigger_invisible_ship(
    _: Trigger<InvisibleEvent>,
    mut commands: Commands,
    spaceship_query: Query<Entity, With<Spaceship>>,
) {
    let entity = spaceship_query.get_single().unwrap();
    commands.entity(entity).insert(Invisible {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    });
}

fn apply_invisible(
    mut commands: Commands,
    mut invisible_query: Query<(Entity, &mut Sprite, &mut Invisible)>,
    time: Res<Time>,
) {
    if invisible_query.is_empty() {
        return;
    }
    for (entity, mut sprite, mut invisible) in invisible_query.iter_mut() {
        invisible.timer.tick(time.delta());
        if invisible.timer.finished() {
            sprite.color.set_alpha(1.);
            commands.entity(entity).remove::<Invisible>();
            continue;
        }
        if sprite.color.alpha() >= 1. {
            sprite.color.set_alpha(0.)
        } else {
            sprite.color.set_alpha(1.)
        }
    }
}
