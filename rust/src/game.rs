use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct Game {
    name: String,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }

    fn new(_owner: &Node) -> Self {
        godot_print!("Game is created!");
        Game {
            name: "".to_string(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {
        let ecs = owner.get_node("/root/ECSController").unwrap();
        let ecs = unsafe { ecs.assume_safe() };
        unsafe { ecs.call("add_game_to_ecs", &[Variant::from_object(owner)]) };
    }
}
