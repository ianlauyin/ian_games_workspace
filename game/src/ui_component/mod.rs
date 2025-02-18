mod blink;
mod interaction_ui;
mod main_container;
mod selectable_text;

pub use blink::Blink;
pub use interaction_ui::InteractionUI;
pub use main_container::MainContainer;
pub use selectable_text::SelectableText;

use bevy::prelude::{App, Plugin};
pub struct UIComponentPlugin;

impl Plugin for UIComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            blink::BlinkPlugin,
            main_container::MainContainerPlugin,
            selectable_text::SelectableTextPlugin,
            interaction_ui::InteractionUIPlugin,
        ));
    }
}
