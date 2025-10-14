use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;

use crate::ui_components::Blink;

#[derive(Component)]
#[require(Sprite)]
pub struct Invisible {
    timer: Timer,
}

impl Invisible {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
        }
    }
}

pub struct InvisiblePlugin;

impl Plugin for InvisiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_invisible_timer)
            .add_observer(invisible_on_add);
    }
}

fn invisible_on_add(ev: On<Add, Invisible>, mut commands: Commands) {
    commands
        .entity(ev.entity)
        .insert(Blink::new_with_speed(1.1));
}

fn handle_invisible_timer(
    mut commands: Commands,
    mut invisible_query: Query<(Entity, &mut Invisible)>,
    time: Res<Time>,
) {
    for (entity, mut invisible) in invisible_query.iter_mut() {
        invisible.timer.tick(time.delta());
        if invisible.timer.is_finished() {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<Invisible>();
                entity_commands.remove::<Blink>();
            }
        }
    }
}
