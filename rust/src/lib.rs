mod ecs;
mod ecs_controller;
mod game;
mod player;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ecs_controller::ECSController>();
    handle.add_class::<game::Game>();
    handle.add_class::<player::Player>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
