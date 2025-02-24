use std::time::Duration;

use bevy::prelude::*;

use crate::constant::ZIndex;
use crate::constant::SPACESHIP_SIZE;
use crate::res::ImageHandles;

use super::collisable::Collisable;

#[derive(Component)]
#[require(Collisable)]
pub struct Spaceship {
    position: Vec2,
    cooldown: Option<Timer>,
}

impl Spaceship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            cooldown: None,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn can_shoot(&self) -> bool {
        self.cooldown.is_none()
    }

    pub fn start_cd(&mut self) {
        self.cooldown = Some(Timer::new(Duration::from_millis(100), TimerMode::Once));
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (listen_spaceship_position, handle_cooldown))
            .add_observer(handle_spaceship_on_added);
    }
}

fn handle_spaceship_on_added(
    ev: Trigger<OnAdd, Spaceship>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    spaceship_query: Query<&Spaceship>,
) {
    let spaceship = spaceship_query.get(ev.entity()).unwrap();
    commands.entity(ev.entity()).insert((
        Sprite {
            image: image_handles.spaceship.clone(),
            custom_size: Some(SPACESHIP_SIZE),
            ..default()
        },
        Transform::from_translation(spaceship.position.extend(ZIndex::SPACESHIP.z_value())),
    ));
}

fn listen_spaceship_position(mut spaceship_query: Query<(&Transform, &mut Spaceship)>) {
    for (transform, mut spaceship) in spaceship_query.iter_mut() {
        spaceship.position = transform.translation.xy();
    }
}

fn handle_cooldown(mut spaceship_query: Query<&mut Spaceship>, time: Res<Time>) {
    for mut spaceship in spaceship_query.iter_mut() {
        if let Some(timer) = &mut spaceship.cooldown {
            timer.tick(time.delta());
            if timer.finished() {
                spaceship.cooldown = None;
            }
        };
    }
}
