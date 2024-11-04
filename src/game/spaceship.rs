use std::time::Duration;

use bevy::prelude::*;

use crate::asset_loader::ImageHandles;
use crate::game::ShootBulletEvent;
use crate::states::{AppState, GameState};
use crate::ui::{
    get_bottom_edge, get_left_edge, get_right_edge, get_spaceship_size, get_top_edge, ZIndexMap,
};
use crate::util::Velocity;

#[derive(Component)]
pub struct Spaceship {
    bullet_cd: Option<Timer>,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_spaceship)
            .add_systems(
                Update,
                check_spaceship_position.run_if(in_state(GameState::Ready)),
            )
            .add_systems(
                FixedUpdate,
                (
                    handle_spaceship_interaction,
                    (handle_bullet_cooldown, handle_shoot_bullet).chain(),
                )
                    .run_if(in_state(GameState::InPlay)),
            );
    }
}

fn setup_spaceship(
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    commands.spawn((
        Spaceship { bullet_cd: None },
        Velocity { x: 0., y: 5. },
        SpriteBundle {
            texture: image_handles.spaceship.clone(),
            sprite: Sprite {
                custom_size: Some(get_spaceship_size(window.width())),
                ..default()
            },
            transform: Transform::from_xyz(
                0.,
                -window.height() / 1.5,
                ZIndexMap::SpaceShip.value(),
            ),
            ..default()
        },
    ));
}

fn check_spaceship_position(
    mut next_state: ResMut<NextState<GameState>>,
    mut spaceship_query: Query<(&Transform, &mut Velocity), With<Spaceship>>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap();
    let (transform, mut velocity) = spaceship_query.get_single_mut().unwrap();
    if transform.translation.y >= -window.height() / 2.5 {
        velocity.y = 0.;
        next_state.set(GameState::InPlay);
    }
}

fn handle_spaceship_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut spaceship_query: Query<(&mut Velocity, &Transform), With<Spaceship>>,
    windows: Query<&Window>,
) {
    let (mut velocity, transform) = spaceship_query.get_single_mut().unwrap();
    let window = windows.get_single().unwrap();

    velocity.y = match (
        keys.pressed(KeyCode::ArrowUp),
        keys.pressed(KeyCode::ArrowDown),
    ) {
        (true, false)
            if transform.translation.y
                <= get_top_edge(window.height(), get_spaceship_size(window.width()).y) =>
        {
            10.
        }
        (false, true)
            if transform.translation.y
                >= get_bottom_edge(window.height(), get_spaceship_size(window.width()).y) =>
        {
            -10.
        }
        _ => 0.,
    };

    velocity.x = match (
        keys.pressed(KeyCode::ArrowLeft),
        keys.pressed(KeyCode::ArrowRight),
    ) {
        (false, true)
            if transform.translation.x
                <= get_right_edge(window.width(), get_spaceship_size(window.width()).x) =>
        {
            10.
        }
        (true, false)
            if transform.translation.x
                >= get_left_edge(window.width(), get_spaceship_size(window.width()).x) =>
        {
            -10.
        }
        _ => 0.,
    };
}

fn handle_shoot_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut spaceship_query: Query<(&Transform, &mut Spaceship)>,
) {
    if keys.pressed(KeyCode::Space) {
        let (transform, mut spaceship) = spaceship_query.get_single_mut().unwrap();
        let Vec3 { x, y, .. } = transform.translation;
        if spaceship.bullet_cd.is_none() {
            commands.trigger(ShootBulletEvent { x, y });
            spaceship.bullet_cd = Some(Timer::new(Duration::from_millis(100), TimerMode::Once));
        }
    }
}

fn handle_bullet_cooldown(mut spaceship_query: Query<&mut Spaceship>, time: Res<Time>) {
    let mut spaceship = spaceship_query.get_single_mut().unwrap();
    let Some(ref mut timer) = &mut spaceship.bullet_cd else {
        return;
    };
    timer.tick(time.delta());
    if timer.finished() {
        spaceship.bullet_cd = None;
    }
}
