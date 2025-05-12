use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;
use rand::{rng, Rng};

use crate::{
    constant::{ZIndex, BULLET_SIZE},
    res::PlayerTag,
    util::{listen_position, Position},
};

use super::{collisable::Collisable, Player, Velocity};

// Only SelfPlayer can have this component
#[derive(Component)]
pub struct BulletTag(pub u16);

#[derive(Component)]
pub struct Bullet {
    player: u8,
    position: Vec2,
}

impl Position for Bullet {
    fn get_position(&self) -> Vec2 {
        self.position
    }
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

impl Bullet {
    pub fn by_player(player: u8, position: Vec2) -> Self {
        Self { player, position }
    }
    pub fn get_player(&self) -> u8 {
        self.player
    }
    pub fn get_position_tuple(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_position::<Bullet>)
            .add_observer(bullet_on_added);
    }
}

fn bullet_on_added(
    ev: Trigger<OnAdd, Bullet>,
    mut commands: Commands,
    bullet_q: Query<&Bullet>,
    player_tag: Res<PlayerTag>,
) {
    let bullet = bullet_q.get(ev.target()).unwrap();
    let color = if bullet.get_player() == player_tag.0 {
        Color::from(YELLOW)
    } else {
        Color::srgb(0.5, 0.5, 0.)
    };

    if let Ok(mut entity_commands) = commands.get_entity(ev.target()) {
        entity_commands.insert((
            Velocity { x: 0., y: 10. },
            Transform::from_translation(bullet.get_position().extend(ZIndex::BULLET.z_value())),
            Sprite {
                color,
                custom_size: Some(BULLET_SIZE),
                ..default()
            },
            Player(bullet.get_player()),
        ));
        if bullet.get_player() == player_tag.0 {
            let bullet_tag = rng().random_range(u16::MIN..u16::MAX);
            entity_commands.insert((Collisable::Player, BulletTag(bullet_tag)));
        }
    }
}
