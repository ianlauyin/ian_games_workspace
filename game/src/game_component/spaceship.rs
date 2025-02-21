use bevy::prelude::*;

use crate::constant::ZIndex::SPACESHIP;
use crate::constant::SPACESHIP_SIZE;
use crate::res::ImageHandles;

#[derive(Component)]
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
        app.add_observer(handle_spaceship_on_added);
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
        Transform::from_translation(spaceship.position.extend(SPACESHIP.z_value())),
    ));
}

// fn check_spaceship_position(
//     mut next_state: ResMut<NextState<GameState>>,
//     mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
//     windows: Query<&Window>,
// ) {
//     let window = windows.get_single().unwrap();
//     let (transform, mut velocity) = spaceship_query.get_single_mut().unwrap();
//     if transform.translation.y >= -window.height() / 2. + SPACESHIP_SIZE.y {
//         velocity.y = 0.;
//         next_state.set(GameState::InPlay);
//     }
// }

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
