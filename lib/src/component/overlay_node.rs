use bevy::prelude::*;

#[derive(Component)]
pub struct OverlayNode;

pub struct OverlayNodePlugin;

impl Plugin for OverlayNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_overlay_node_on_add);
    }
}

fn handle_overlay_node_on_add(ev: On<Add, OverlayNode>, mut commands: Commands) {
    if let Ok(mut entity_commands) = commands.get_entity(ev.entity) {
        entity_commands.insert((
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                padding: UiRect::all(Val::Px(10.)),
                row_gap: Val::Px(5.),
                ..default()
            },
            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.5)),
        ));
    }
}
