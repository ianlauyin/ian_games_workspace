use bevy::prelude::*;

mod flow;

fn main() {
    App::new().add_plugins(flow::FlowPlugin).run();
}
