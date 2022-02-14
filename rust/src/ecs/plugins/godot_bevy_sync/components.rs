use bevy::{ecs::world::EntityMut, prelude::*};
use gdnative::api::*;
use gdnative::prelude::*;

#[derive(Component)]
pub struct GodotObjRef<T: GodotObject>(pub Ref<T>);

#[derive(Component)]
pub struct GodotSceneTree(pub Ref<SceneTree>);

#[derive(Component)]
pub struct GameNode;

pub fn insert_godot_ref<'a, T: 'static + SubClass<Node>>(
    node: TRef<'a, Node>,
    entity_mut: &mut EntityMut,
) -> TRef<'a, T> {
    let node = node.cast::<T>().expect(&format!(
        "Expected node to be castable into {}",
        node.get_class().to_string()
    ));
    entity_mut.insert(GodotObjRef(node.claim()));
    node
}
