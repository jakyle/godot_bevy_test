use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player {
    #[export]
    speed: f64,
    name: String,
}

#[methods]
impl Player {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
    }

    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("Player is created!");
        Player {
            speed: 0.0,
            name: "".to_string(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.name = "Player".to_string();
        godot_print!("{} is ready!", self.name);
    }

    #[export]
    unsafe fn _process(&self, _owner: &KinematicBody2D, delta: f64) {
        godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}
