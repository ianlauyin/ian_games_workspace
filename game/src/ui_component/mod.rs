mod blink;

pub use blink::{Blink};

use bevy::prelude::{App, Plugin};
pub struct UIComponentPlugin;

impl Plugin for UIComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(blink::BlinkPlugin);
    }
}
