use bevy::app::App;
use bevy::prelude::*;

use crate::util::angle_to_radian;

pub struct ControlButtonPlugin;

impl Plugin for ControlButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_control_button_on_add);
    }
}

#[derive(Component)]
pub struct ControlButtonPanel;

#[derive(Component)]
pub enum ControlButton {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn handle_control_button_on_add(ev: On<Add, ControlButtonPanel>, mut commands: Commands) {
    let button_layout_vec = [
        Some((
            ControlButton::UpLeft,
            Quat::from_rotation_z(angle_to_radian(-135.)),
        )),
        Some((
            ControlButton::Up,
            Quat::from_rotation_z(angle_to_radian(-90.)),
        )),
        Some((
            ControlButton::UpRight,
            Quat::from_rotation_z(angle_to_radian(-45.)),
        )),
        Some((
            ControlButton::Left,
            Quat::from_rotation_z(angle_to_radian(180.)),
        )),
        None,
        Some((ControlButton::Right, Quat::from_rotation_z(0.))),
        Some((
            ControlButton::DownLeft,
            Quat::from_rotation_z(angle_to_radian(135.)),
        )),
        Some((
            ControlButton::Down,
            Quat::from_rotation_z(angle_to_radian(90.)),
        )),
        Some((
            ControlButton::DownRight,
            Quat::from_rotation_z(angle_to_radian(45.)),
        )),
    ];
    commands
        .entity(ev.entity)
        .insert((Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            bottom: Val::Px(20.),
            width: Val::Px(300.),
            height: Val::Px(300.),
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::auto(3)],
            grid_template_rows: vec![RepeatedGridTrack::auto(3)],
            ..default()
        },))
        .with_children(|control_button| {
            for button_layout in button_layout_vec {
                let Some((button_direction, rotation)) = button_layout else {
                    control_button.spawn(Node::default());
                    continue;
                };
                control_button
                    .spawn((
                        button_direction,
                        Interaction::default(),
                        Node {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        BackgroundColor::from(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                        BorderRadius::all(Val::Px(5.)),
                    ))
                    .with_child((Text::new(">"), Transform::from_rotation(rotation)));
            }
        });
}
