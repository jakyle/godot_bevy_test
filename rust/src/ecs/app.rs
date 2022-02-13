use bevy::{
    core::FixedTimestep,
    prelude::{App, CoreStage, SystemSet, SystemStage},
    MinimalPlugins,
};

use super::{
    godot_bevy_sync::{
        events::UserInput,
        resources::{IdleDelta, PhysicsDelta},
        stages::SyncStages,
    },
    plugins::movement::MovementPlugin,
};

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_event::<UserInput>()
        .add_stage_after(
            CoreStage::Last,
            SyncStages::UpdateBevy,
            SystemStage::parallel(),
        )
        .add_stage_after(
            SyncStages::UpdateBevy,
            SyncStages::UpdateBevyPhysics,
            SystemStage::single_threaded(),
        )
        .add_plugins(MinimalPlugins)
        .add_plugin(MovementPlugin)
        .init_resource::<IdleDelta>()
        .init_resource::<PhysicsDelta>();

    ecs
}
