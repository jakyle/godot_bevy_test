use super::resources::Delta;
use crate::ecs::plugins::movement::{SpawnMovementCrab, SpawnMovementPlayer};
use bevy::{ecs::system::Resource, prelude::World};
use gdnative::{
    api::{InputEvent, KinematicBody2D},
    Ref, TRef,
};

pub const PRESSED_ACTIONS: [&str; 4] = ["ui_left", "ui_right", "ui_up", "ui_down"];

pub struct UserInput(pub Action);

pub enum Action {
    Pressed(&'static str),
    Released(&'static str),
}

pub fn update_delta_resource<T: Resource + Delta>(world: &mut World, delta: f32) {
    world
        .get_resource_mut::<T>()
        .expect("Umm, there should be a godot time here!")
        .set_delta(delta);
}

pub fn user_input(world: &mut World, event: TRef<InputEvent>) {
    let mut send_user_input = world
        .get_resource_mut::<bevy::app::Events<UserInput>>()
        .expect("should be a user input event");
    for action in PRESSED_ACTIONS.iter() {
        if event.is_action(action) {
            if event.is_pressed() && !event.is_echo() {
                send_user_input.send(UserInput(Action::Pressed(action)));
            } else if !event.is_pressed() && !event.is_echo() {
                send_user_input.send(UserInput(Action::Released(action)));
            }
        }
    }
}

pub fn spawn_movement_crab(world: &mut World, node: Ref<KinematicBody2D>, speed: f32) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnMovementCrab>>()
        .expect(
            "No Events<E> resource found. Did you forget to call `.init_resource` or `.add_event`?",
        )
        .send(SpawnMovementCrab((node.clone(), speed)));
}

pub fn spawn_movement_player(world: &mut World, node: Ref<KinematicBody2D>, speed: f32) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnMovementPlayer>>()
        .expect(
            "No Events<E> resource found. Did you forget to call `.init_resource` or `.add_event`?",
        )
        .send(SpawnMovementPlayer((node.clone(), speed)));
}
