use bevy::{prelude::*, log::LogPlugin, diagnostic::DiagnosticsPlugin};

use super::plugins::{godot_bevy_sync::GodotBevySyncPlugin, movement::MovementPlugin};

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(GodotBevySyncPlugin)  // order matters here
        .add_plugin(MovementPlugin);

    ecs
}
