use bevy::prelude::*;
use gdnative::{api::AudioStreamPlayer2D, prelude::*};

use rand::prelude::*;

use super::engine_sync::{
    components::{GodotObjRef, PlayingGame},
    events::UserInput,
    resources::{PhysicsDelta, GameOver},
    stages::SyncStages,
};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnMovementPlayer>()
            .add_event::<SpawnMovementCrab>()
            .insert_resource(InputVector(Vector2::zero()))
            .add_system_to_stage(CoreStage::PreUpdate, on_movement_input)
            .add_system(spawn_movement_crab)
            .add_system(spawn_movement_player)
            .add_system(move_player)
            .add_system_to_stage(SyncStages::UpdateBevy, play_move_sound)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, move_player_sync)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, move_crab_sync);
    }
}

pub struct InputVector(pub Vector2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Crab;

#[derive(Component)]
pub struct MoveSound(GodotObjRef<AudioStreamPlayer2D>);

#[derive(Component)]
pub struct Velocity(pub Vector2);

#[derive(Component)]
pub struct Speed(f32);

pub struct SpawnMovementPlayer(pub (Ref<KinematicBody2D>, f32));
pub struct SpawnMovementCrab(pub (Ref<KinematicBody2D>, f32));

fn spawn_movement_player(
    mut commands: Commands,
    mut on_spawn_movement_player: EventReader<SpawnMovementPlayer>,
) {
    for SpawnMovementPlayer((kb2, speed)) in on_spawn_movement_player.iter() {
        let movement_sound = unsafe {
            kb2.assume_safe()
                .get_node("MoveSound")
                .unwrap()
                .assume_safe()
                .cast::<AudioStreamPlayer2D>()
                .unwrap()
                .assume_shared()
        };

        commands
            .spawn()
            .insert(GodotObjRef(kb2.clone()))
            .insert(Player)
            .insert(Speed(*speed))
            .insert(Velocity(Vector2::new(0.0, 0.0)))
            .insert(PlayingGame)
            .insert(MoveSound(GodotObjRef(movement_sound)));
    }
}

fn spawn_movement_crab(
    mut commands: Commands,
    mut on_spawn_movement_player: EventReader<SpawnMovementCrab>,
) {
    for SpawnMovementCrab((kb2, speed)) in on_spawn_movement_player.iter() {
        let mut rng = rand::thread_rng();

        let (x, y) = (rng.gen_range(-0.01..0.01), rng.gen_range(-0.01..0.01));

        let velocity = normalized(Vector2::new(x, y)) * *speed;

        commands
            .spawn()
            .insert(GodotObjRef(kb2.clone()))
            .insert(Crab)
            .insert(PlayingGame)
            .insert(Velocity(velocity));
    }
}

const ACCELERATION: f32 = 100.0;
const MAX_SPEED: f32 = 16.0;

fn on_movement_input(
    mut input_vector: ResMut<InputVector>,
    mut on_movement_input: EventReader<UserInput>,
) {
    for UserInput(action) in on_movement_input.iter() {
        match *action {
            crate::ecs::plugins::engine_sync::events::Action::Pressed(msg) => match msg {
                "ui_left" => input_vector.0 += Vector2::new(-1.0, 0.0),
                "ui_right" => input_vector.0 += Vector2::new(1.0, 0.0),
                "ui_down" => input_vector.0 += Vector2::new(0.0, 1.0),
                "ui_up" => input_vector.0 += Vector2::new(0.0, -1.0),
                _ => (),
            },
            crate::ecs::plugins::engine_sync::events::Action::Released(msg) => match msg {
                "ui_left" => input_vector.0 -= Vector2::new(-1.0, 0.0),
                "ui_right" => input_vector.0 -= Vector2::new(1.0, 0.0),
                "ui_down" => input_vector.0 -= Vector2::new(0.0, 1.0),
                "ui_up" => input_vector.0 -= Vector2::new(0.0, -1.0),
                _ => (),
            },
        }
    }
}

fn move_player(
    delta: Res<PhysicsDelta>,
    input_vector: Res<InputVector>,
    mut query: Query<(&mut Velocity, &Speed), (With<Player>, With<GodotObjRef<KinematicBody2D>>)>,
) {
    for (mut velocity, speed) in query.iter_mut() {
        velocity.0 = velocity.0.move_towards(
            normalized(input_vector.0) * MAX_SPEED * speed.0,
            ACCELERATION * delta.0 as f32,
        );
    }
}

fn play_move_sound(
    mut query: Query<(&MoveSound, &Velocity), With<Player>>,

) {
        for (move_sound, velocity) in query.iter_mut() {
    
            let move_sound = unsafe { move_sound.0.0.assume_safe() };

            let zero = Vector2::zero();

            if velocity.0 != zero && !move_sound.is_playing() {
                move_sound.play(0.0);
            } else if velocity.0 == zero && move_sound.is_playing() {
                move_sound.stop();
            }
        }
    

}


#[inline]
pub fn normalized(vector_to_normalize: Vector2) -> Vector2 {
    let option = Vector2::try_normalize(vector_to_normalize);
    match option {
        None => Vector2::zero(),
        Some(vector2) => vector2,
    }
}

fn move_crab_sync(
    mut game_over: ResMut<Option<GameOver>>,
    mut crabs: Query<(&GodotObjRef<KinematicBody2D>, &mut Velocity), With<Crab>>,
) {
    for (kb2, mut velocity) in crabs.iter_mut() {
        unsafe {
            if let Some(tref_kb2d) = kb2.0.assume_safe_if_sane() {
                let collider = tref_kb2d.move_and_collide(velocity.0, true, true, false);

                if let Some(collider) = collider {
                    let collision = collider.assume_safe();

                    let is_player = collision
                        .collider()
                        .unwrap()
                        .assume_safe()
                        .cast::<Node2D>()
                        .unwrap()
                        .name()
                        .to_string()
                        .contains("Player");

                    if is_player {
                        *game_over = Some(GameOver::Lose);
                    }

                    velocity.0 = velocity.0.reflect(collision.normal());
                }
            }
        }
    }
}

fn move_player_sync(
    mut game_over: ResMut<Option<GameOver>>,
    mut query: Query<(&GodotObjRef<KinematicBody2D>, &Velocity), With<Player>>,
) {
    for (kb2, velocity) in query.iter_mut() {
        unsafe {
            if let Some(tref_kb2d) = kb2.0.assume_safe_if_sane() {
                for i in 0..tref_kb2d.get_slide_count() {
                    let is_crab = tref_kb2d
                        .get_slide_collision(i)
                        .unwrap()
                        .assume_safe()
                        .collider()
                        .unwrap()
                        .assume_safe()
                        .cast::<Node2D>()
                        .unwrap()
                        .name()
                        .to_string()
                        .contains("rab");

                    if is_crab {
                        *game_over = Some(GameOver::Lose);
                    }
                }

                tref_kb2d.move_and_slide(
                    velocity.0,
                    Vector2::zero(),
                    false,
                    4,
                    std::f64::consts::FRAC_PI_4,
                    true,
                );
            }
        }
    }
}
