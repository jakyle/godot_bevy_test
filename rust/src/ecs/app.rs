use bevy::{diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*};

use super::plugins::{
    countdown::CountdownPlugin, engine_sync::EngineSyncPlugin, movement::MovementPlugin,
};

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(EngineSyncPlugin) // order matters here
        .add_plugin(MovementPlugin)
        .add_plugin(CountdownPlugin);

    ecs
}
