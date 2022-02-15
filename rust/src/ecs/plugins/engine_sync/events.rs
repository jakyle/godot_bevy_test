use crate::ecs::plugins::movement::{SpawnMovementCrab, SpawnMovementPlayer};
use bevy::{ecs::system::Resource, prelude::World};
use gdnative::{
    api::{InputEvent, KinematicBody2D, Label, Node},
    Ref, TRef,
};

use super::resources::Delta;

pub const PRESSED_ACTIONS: [&str; 4] = ["ui_left", "ui_right", "ui_up", "ui_down"];

pub struct SpawnGame(pub Ref<Node>);

pub struct SpawnTimer(pub Ref<Label>);

pub struct SpawnTitleMenu(pub Ref<Node>);

pub struct DespawnPlayingGame;

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
pub fn spawn_game(world: &mut World, node: Ref<Node>) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnGame>>()
        .expect("No world spawn game event, did you forget to add Spawn Game into your events?")
        .send(SpawnGame(node));
}

pub fn spawn_timer(world: &mut World, node: Ref<Label>) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnTimer>>()
        .expect("No timer spawn game event,did you forget to add the spawn timer event?")
        .send(SpawnTimer(node));
}

pub fn spawn_title_menu(world: &mut World, node: Ref<Node>) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnTitleMenu>>()
        .expect("No event for spawning title menu, did you forget to add it?")
        .send(SpawnTitleMenu(node));
}
