use bevy::prelude::*;

use crate::constant::{ZIndex, UFO_SIZE};
use crate::res::ImageHandles;
use crate::util::EdgeUtil;

use super::collisable::Collisable;

#[derive(Component)]
#[require(Collisable)]
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
        app.add_systems(Update, (listen_ufo_position, cleanup_on_out_screen))
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
    commands.entity(ev.entity()).insert((
        Sprite {
            image: image_handles.ufo.clone(),
            custom_size: Some(UFO_SIZE),
            ..default()
        },
        Transform::from_translation(ufo.position.extend(ZIndex::UFO.z_value())),
    ));
}

fn listen_ufo_position(mut ufo_query: Query<(&Transform, &mut UFO)>) {
    for (transform, mut ufo) in ufo_query.iter_mut() {
        ufo.position = transform.translation.xy();
    }
}

fn cleanup_on_out_screen(
    mut commands: Commands,
    ufo_query: Query<(Entity, &Transform), With<UFO>>,
) {
    let edge = EdgeUtil::new(UFO_SIZE);
    for (entity, transform) in ufo_query.iter() {
        if edge.over_bottom_out(transform.translation.y) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
