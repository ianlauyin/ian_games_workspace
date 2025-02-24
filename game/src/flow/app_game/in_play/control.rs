use bevy::prelude::*;

use super::super::triggers::{SpaceShipMovement, SpaceShipMovementEvent};
use crate::ui_components::{ControlButton, ControlButtonPanel};
use crate::{
    res::{ControlMode, ControlOption},
    states::GameState,
};
pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlay), spawn_control_button_panel)
            .add_systems(
                Update,
                (
                    handle_clicking_interaction,
                    handle_spaceship_keyboard_interaction,
                )
                    .run_if(in_state(GameState::InPlay)),
            );
    }
}

fn spawn_control_button_panel(mut commands: Commands, control_option: Res<ControlOption>) {
    if control_option.mode == ControlMode::Keyboard {
        return;
    }
    commands.spawn(ControlButtonPanel);
}

// Button Mode
fn handle_clicking_interaction(
    mut commands: Commands,
    mut control_button_query: Query<(&Interaction, &mut BackgroundColor, &ControlButton)>,
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
                ControlButton::Up => SpaceShipMovement::Up,
                ControlButton::UpRight => SpaceShipMovement::UpRight,
                ControlButton::Right => SpaceShipMovement::Right,
                ControlButton::DownRight => SpaceShipMovement::DownRight,
                ControlButton::Down => SpaceShipMovement::Down,
                ControlButton::DownLeft => SpaceShipMovement::DownLeft,
                ControlButton::Left => SpaceShipMovement::Left,
                ControlButton::UpLeft => SpaceShipMovement::UpLeft,
            };
            commands.trigger(SpaceShipMovementEvent {
                movement,
                player: 1,
            });
        }
        background_color.0.set_alpha(0.5);
    }
    if all_not_pressed {
        commands.trigger(SpaceShipMovementEvent {
            movement: SpaceShipMovement::Rest,
            player: 1,
        });
    }
}

// Keyboard Mode
fn handle_spaceship_keyboard_interaction(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    control_option: Res<ControlOption>,
) {
    if control_option.mode != ControlMode::Keyboard {
        return;
    }
    let movement = match (
        keys.pressed(KeyCode::ArrowUp),
        keys.pressed(KeyCode::ArrowDown),
        keys.pressed(KeyCode::ArrowLeft),
        keys.pressed(KeyCode::ArrowRight),
    ) {
        (true, false, true, false) => SpaceShipMovement::UpLeft,
        (true, false, false, true) => SpaceShipMovement::UpRight,
        (false, true, true, false) => SpaceShipMovement::DownLeft,
        (false, true, false, true) => SpaceShipMovement::DownRight,
        (true, false, _, _) => SpaceShipMovement::Up,
        (false, true, _, _) => SpaceShipMovement::Down,
        (_, _, true, false) => SpaceShipMovement::Left,
        (_, _, false, true) => SpaceShipMovement::Right,
        _ => SpaceShipMovement::Rest,
    };
    commands.trigger(SpaceShipMovementEvent {
        movement,
        player: 1,
    });
}
