use bevy::prelude::{CoreStage, Plugin, SystemStage, SystemSet, ParallelSystemDescriptorCoercion};

use self::{
    events::{SpawnGame, SpawnTimer, UserInput, SpawnTitleMenu, DespawnPlayingGame},
    resources::{IdleDelta, PhysicsDelta, GameOver},
    stages::SyncStages,
    systems::{end_game, spawn_game, spawn_title_menu, despawn_playing_game},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod stages;
pub mod systems;

pub struct EngineSyncPlugin;

impl Plugin for EngineSyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInput>()
            .add_event::<SpawnGame>()
            .add_event::<SpawnTimer>()
            .add_event::<SpawnTitleMenu>()
            .add_event::<DespawnPlayingGame>()
            .add_system_to_stage(CoreStage::Last, spawn_game) 
            .add_system_to_stage(CoreStage::Last, spawn_title_menu) 
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
            .add_system_to_stage(SyncStages::UpdateBevy, end_game.label("end_game"))
            .add_system(despawn_playing_game.after("end_game"))
            .init_resource::<IdleDelta>()
            .init_resource::<PhysicsDelta>()
            .init_resource::<Option<GameOver>>();
    }
}
