use bevy::prelude::*;

use crate::constant::ZIndex;
use crate::constant::SPACESHIP_SIZE;
use crate::res::ImageHandles;

use super::collisable::Collisable;

#[derive(Component)]
#[require(Collisable)]
pub struct Spaceship {
    position: Vec2,
    bullet_cd: Option<Timer>,
}

impl Spaceship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            bullet_cd: None,
        }
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_spaceship_position)
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

// fn handle_shoot_bullet(
//     mut commands: Commands,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut spaceship_query: Query<(&Transform, &mut Spaceship)>,
//     control_option: Res<ControlOption>,
// ) {
//     if keys.pressed(KeyCode::Space) || control_option.mode == ControlMode::Button {
//         let (transform, mut spaceship) = spaceship_query.get_single_mut().unwrap();
//         let Vec3 { x, y, .. } = transform.translation;
//         if spaceship.bullet_cd.is_none() {
//             commands.trigger(ShootBulletEvent { x, y });
//             spaceship.bullet_cd = Some(Timer::new(Duration::from_millis(200), TimerMode::Once));
//         }
//     }
// }

// fn handle_bullet_cooldown(mut spaceship_query: Query<&mut Spaceship>, time: Res<Time>) {
//     let mut spaceship = spaceship_query.get_single_mut().unwrap();
//     let Some(ref mut timer) = &mut spaceship.bullet_cd else {
//         return;
//     };
//     timer.tick(time.delta());
//     if timer.finished() {
//         spaceship.bullet_cd = None;
//     }
// }
