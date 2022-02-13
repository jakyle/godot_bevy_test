use bevy::prelude::StageLabel;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, StageLabel)]
pub enum SyncStages {
    UpdateBevy,
    UpdateBevyPhysics,
}
