use bevy::prelude::*;
pub struct ControlPanelPlugin;

impl Plugin for ControlPanelPlugin {
    fn build(&self, app: &mut App) {}
}

// fn handle_clicking_interaction(
//     mut commands: Commands,
//     mut control_button_query: Query<(&Interaction, &mut BackgroundColor, &ControlButton)>,
//     control_option: Res<ControlOption>,
// ) {
//     if control_option.mode == ControlMode::Keyboard {
//         return;
//     }
//     let mut all_not_pressed = true;
//     for (interaction, mut background_color, button_direction) in control_button_query.iter_mut() {
//         if *interaction == Interaction::Pressed {
//             background_color.0.set_alpha(0.1);
//             all_not_pressed = false;
//             let movement = match button_direction {
//                 ControlButton::Up => SpaceShipMovement::Up,
//                 ControlButton::UpRight => SpaceShipMovement::UpRight,
//                 ControlButton::Right => SpaceShipMovement::Right,
//                 ControlButton::DownRight => SpaceShipMovement::DownRight,
//                 ControlButton::Down => SpaceShipMovement::Down,
//                 ControlButton::DownLeft => SpaceShipMovement::DownLeft,
//                 ControlButton::Left => SpaceShipMovement::Left,
//                 ControlButton::UpLeft => SpaceShipMovement::UpLeft,
//             };
//             commands.trigger(SpaceShipMovementEvent(movement));
//         }
//         background_color.0.set_alpha(0.5);
//     }
//     if all_not_pressed {
//         commands.trigger(SpaceShipMovementEvent(SpaceShipMovement::Rest));
//     }
// }
