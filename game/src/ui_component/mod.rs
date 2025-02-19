mod blink;
mod control_button_panel;
mod interaction_ui;
mod main_container;
mod selectable_text;
mod velocity;

pub use blink::Blink;
pub use control_button_panel::{ControlButton, ControlButtonPanel};
pub use interaction_ui::InteractionUI;
pub use main_container::MainContainer;
pub use selectable_text::SelectableText;
pub use velocity::Velocity;

use bevy::prelude::{App, Plugin};
pub struct UIComponentPlugin;

impl Plugin for UIComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            blink::BlinkPlugin,
            control_button_panel::ControlButtonPlugin,
            main_container::MainContainerPlugin,
            selectable_text::SelectableTextPlugin,
            interaction_ui::InteractionUIPlugin,
            velocity::VelocityPlugin,
        ));
    }
}
