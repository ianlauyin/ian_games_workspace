use std::time::Duration;

use bevy::prelude::*;
use shooting_game_shared::util::SPACESHIP_SIZE;

use crate::constant::ZIndex;
use crate::res::ImageHandles;
use crate::res::PlayerTag;
use crate::util::listen_position;
use crate::util::Position;

use super::collisable::Collisable;
use super::Player;

#[derive(Component)]
pub struct Spaceship {
    position: Vec2,
    cooldown: Option<Timer>,
}

impl Position for Spaceship {
    fn get_position(&self) -> Vec2 {
        self.position
    }
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

impl Spaceship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            cooldown: None,
        }
    }

    pub fn get_position_tuple(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
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
        app.add_systems(Update, (listen_position::<Spaceship>, handle_cooldown))
            .add_observer(handle_spaceship_on_added);
    }
}

fn handle_spaceship_on_added(
    ev: Trigger<OnAdd, Spaceship>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    spaceship_query: Query<(&Player, &Spaceship)>,
    player_tag: Res<PlayerTag>,
) {
    let Ok((player, spaceship)) = spaceship_query.get(ev.target()) else {
        warn!("Player not found in handle_spaceship_on_added");
        return;
    };
    let (z, color) = if player_tag.0 == player.0 {
        (ZIndex::SELFSPACESHIP.z_value(), Color::WHITE)
    } else {
        (ZIndex::SPACESHIP.z_value(), Color::srgb(0.5, 0.5, 0.5))
    };
    if let Ok(mut entity_commands) = commands.get_entity(ev.target()) {
        entity_commands.insert((
            Sprite {
                image: image_handles.spaceship.clone(),
                custom_size: Some(SPACESHIP_SIZE),
                color,
                ..default()
            },
            Transform::from_translation(spaceship.position.extend(z)),
        ));
        if player_tag.0 == player.0 {
            entity_commands.insert(Collisable::Player);
        }
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
