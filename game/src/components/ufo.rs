use crate::constant::ZIndex;
use crate::res::ImageHandles;
use bevy::prelude::*;
use shooting_game_util::UFO_SIZE;

use super::collisable::Collisable;

#[derive(Component)]
pub struct UFO {
    position: Vec2,
}

impl UFO {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
    pub fn get_position(&self) -> Vec2 {
        self.position
    }
}

pub struct UFOPlugin;

impl Plugin for UFOPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_ufo_position)
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

fn listen_ufo_position(mut ufo_query: Query<(&Transform, &mut UFO)>) {
    for (transform, mut ufo) in ufo_query.iter_mut() {
        ufo.position = transform.translation.xy();
    }
}
