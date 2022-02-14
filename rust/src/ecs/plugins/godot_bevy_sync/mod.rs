use bevy::prelude::{CoreStage, Plugin, SystemStage};

use self::{
    events::{EndGame, SpawnGame, UserInput},
    resources::{IdleDelta, PhysicsDelta},
    stages::SyncStages,
    systems::{end_game, spawn_game},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod stages;
pub mod systems;

pub struct GodotBevySyncPlugin;

impl Plugin for GodotBevySyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInput>()
            .add_event::<SpawnGame>()
            .add_event::<EndGame>()
            .add_system_to_stage(CoreStage::Last, spawn_game)
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
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, end_game)
            .init_resource::<IdleDelta>()
            .init_resource::<PhysicsDelta>();
    }
}
