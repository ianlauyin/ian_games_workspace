use std::f32::consts::PI;

use bevy::app::App;
use bevy::prelude::*;

use crate::control::{ControlMode, ControlOption};
use crate::game::{SpaceShipMovement, SpaceShipMovementEvent};
use crate::states::GameState;

pub struct ControlButtonPlugin;

impl Plugin for ControlButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlay), create_control_button)
            .add_systems(
                Update,
                handle_spaceship_clicking_interaction.run_if(in_state(GameState::InPlay)),
            )
            .add_systems(OnExit(GameState::InPlay), clear_up_control_button);
    }
}

#[derive(Component)]
struct ControlButton;

#[derive(Component)]
enum ButtonDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn angle_to_radian(angle: f32) -> f32 {
    angle * PI / 180.
}

fn create_control_button(mut commands: Commands, control_option: Res<ControlOption>) {
    if control_option.mode == ControlMode::Keyboard {
        return;
    }

    let button_layout_vec = [
        Some((
            ButtonDirection::UpLeft,
            Quat::from_rotation_z(angle_to_radian(-135.)),
        )),
        Some((
            ButtonDirection::Up,
            Quat::from_rotation_z(angle_to_radian(-90.)),
        )),
        Some((
            ButtonDirection::UpRight,
            Quat::from_rotation_z(angle_to_radian(-45.)),
        )),
        Some((
            ButtonDirection::Left,
            Quat::from_rotation_z(angle_to_radian(180.)),
        )),
        None,
        Some((ButtonDirection::Right, Quat::from_rotation_z(0.))),
        Some((
            ButtonDirection::DownLeft,
            Quat::from_rotation_z(angle_to_radian(135.)),
        )),
        Some((
            ButtonDirection::Down,
            Quat::from_rotation_z(angle_to_radian(90.)),
        )),
        Some((
            ButtonDirection::DownRight,
            Quat::from_rotation_z(angle_to_radian(45.)),
        )),
    ];
    commands
        .spawn((
            ControlButton,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_self: JustifySelf::Center,
                    bottom: Val::Px(20.),
                    width: Val::Px(300.),
                    height: Val::Px(300.),
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                    grid_template_rows: vec![RepeatedGridTrack::auto(3)],
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|control_button| {
            for button_layout in button_layout_vec {
                let Some((button_direction, rotation)) = button_layout else {
                    control_button.spawn(NodeBundle::default());
                    continue;
                };
                control_button
                    .spawn((
                        button_direction,
                        Interaction::default(),
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            background_color: BackgroundColor::from(Color::srgba(
                                0.3, 0.3, 0.3, 0.5,
                            )),
                            border_radius: BorderRadius::all(Val::Px(5.)),
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text::from_section(">", TextStyle::default()),
                            transform: Transform::from_rotation(rotation),
                            ..default()
                        });
                    });
            }
        });
}

fn handle_spaceship_clicking_interaction(
    mut commands: Commands,
    mut control_button_query: Query<(&Interaction, &mut BackgroundColor, &ButtonDirection)>,
    control_option: Res<ControlOption>,
) {
    if control_option.mode == ControlMode::Keyboard {
        return;
    }
    let mut all_not_pressed = true;
    for (interaction, mut background_color, button_direction) in control_button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            background_color.0.set_alpha(0.1);
            all_not_pressed = false;
            let movement = match button_direction {
                ButtonDirection::Up => SpaceShipMovement::Up,
                ButtonDirection::UpRight => SpaceShipMovement::UpRight,
                ButtonDirection::Right => SpaceShipMovement::Right,
                ButtonDirection::DownRight => SpaceShipMovement::DownRight,
                ButtonDirection::Down => SpaceShipMovement::Down,
                ButtonDirection::DownLeft => SpaceShipMovement::DownLeft,
                ButtonDirection::Left => SpaceShipMovement::Left,
                ButtonDirection::UpLeft => SpaceShipMovement::UpLeft,
            };
            commands.trigger(SpaceShipMovementEvent(movement));
        }
        background_color.0.set_alpha(0.5);
    }
    if all_not_pressed {
        commands.trigger(SpaceShipMovementEvent(SpaceShipMovement::Rest));
    }
}

fn clear_up_control_button(
    mut commands: Commands,
    control_button_query: Query<Entity, With<ControlButton>>,
) {
    let Ok(entity) = control_button_query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}
