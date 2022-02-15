use std::time::Duration;

use bevy::{core::FixedTimestep, prelude::*};
use gdnative::api::Label;

use super::engine_sync::{
    components::{GodotObjRef, PlayingGame},
    events::SpawnTimer,
    resources::{IdleDelta, GameOver},
    stages::SyncStages,
};

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_timer)
            .add_system(tick_down)
            .add_system(end_timer)
            .add_system_set_to_stage(
                SyncStages::UpdateBevy,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(update_label_sync),
            );
    }
}

#[derive(Component)]
pub struct Countdown(pub Timer);

fn spawn_timer(mut commands: Commands, mut on_spawn_timer: EventReader<SpawnTimer>) {
    for SpawnTimer(label) in on_spawn_timer.iter() {
        commands
            .spawn()
            .insert(GodotObjRef(label.clone()))
            .insert(Countdown(Timer::from_seconds(20.0, false)))
            .insert(PlayingGame);
    }
}

fn tick_down(delta: ResMut<IdleDelta>, mut timer: Query<&mut Countdown>) {
    for mut countdown in timer.iter_mut() {
        countdown.0.tick(Duration::from_secs_f32(delta.0));
    }
}

fn end_timer(
    mut game_over: ResMut<Option<GameOver>>,
    timer: Query<&Countdown>
) {
    for countdown in timer.iter() {
        if countdown.0.just_finished() {
            *game_over = Some(GameOver::Win);
        }
    }
}

fn update_label_sync(timer: Query<(&Countdown, &GodotObjRef<Label>)>) {
    for (countdown, label) in timer.iter() {
        if let Some(label) = unsafe { label.0.assume_safe_if_sane() } {
            let time_left = 20.0 - countdown.0.elapsed_secs();
            label.set_text(format!("{time_left:.1}"));
        }
    }
}
