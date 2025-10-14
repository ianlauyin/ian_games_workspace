use bevy::{
    prelude::*,
    window::{CursorIcon, SystemCursorIcon},
};

#[derive(Component, Default)]
#[require(Interaction)]
pub struct InteractionUI;

pub struct InteractionUIPlugin;

impl Plugin for InteractionUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interaction_ui);
    }
}

fn handle_interaction_ui(
    mut commands: Commands,
    mut interaction_ui_query: Query<
        (
            &Interaction,
            Option<&mut BackgroundColor>,
            Option<&mut TextColor>,
        ),
        With<InteractionUI>,
    >,
    windows: Query<Entity, With<Window>>,
) {
    let mut curosr_icon = CursorIcon::default();
    for (interaction, background_color_op, text_color_op) in interaction_ui_query.iter_mut() {
        match interaction {
            Interaction::None => alter_alpha(background_color_op, text_color_op, 1.),
            _ => {
                alter_alpha(background_color_op, text_color_op, 0.5);
                curosr_icon = CursorIcon::System(SystemCursorIcon::Pointer);
            }
        }
    }
    for window in windows.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(window) {
            entity_commands.insert(curosr_icon.clone());
        }
    }
}

fn alter_alpha(
    background_color_op: Option<Mut<BackgroundColor>>,
    text_color_op: Option<Mut<TextColor>>,
    alpha: f32,
) {
    if let Some(mut background_color) = background_color_op {
        background_color.0.set_alpha(alpha);
    }
    if let Some(mut text_color) = text_color_op {
        text_color.set_alpha(alpha);
    }
}
