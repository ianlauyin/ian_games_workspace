use crate::constant::ZIndex;
use crate::res::ImageHandles;
use crate::util::{listen_position, Position};
use bevy::prelude::*;
use shooting_game_shared::util::UFO_SIZE;

use super::collisable::Collisable;

#[derive(Component)]
pub struct UFO {
    position: Vec2,
}

impl Position for UFO {
    fn get_position(&self) -> Vec2 {
        self.position
    }
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

impl UFO {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

pub struct UFOPlugin;

impl Plugin for UFOPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_position::<UFO>)
            .add_observer(handle_ufo_on_added);
    }
}

fn handle_ufo_on_added(
    ev: Trigger<OnAdd, UFO>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    ufo_query: Query<&UFO>,
) {
    let ufo = ufo_query.get(ev.entity()).unwrap();
    if let Some(mut entity_commands) = commands.get_entity(ev.entity()) {
        entity_commands.insert((
            Sprite {
                image: image_handles.ufo.clone(),
                custom_size: Some(UFO_SIZE),
                ..default()
            },
            Transform::from_translation(ufo.position.extend(ZIndex::UFO.z_value())),
            Collisable::Enemy,
        ));
    }
}
