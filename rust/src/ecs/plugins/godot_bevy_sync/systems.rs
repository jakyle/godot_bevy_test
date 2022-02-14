use bevy::prelude::*;
use gdnative::api::*;
use gdnative::prelude::*;

use super::{
    components::{GameNode, GodotObjRef},
    events::{EndGame, SpawnGame},
};

pub fn spawn_game(
    mut commands: Commands,
    mut on_spawn_game: EventReader<SpawnGame>,
    children: Query<Entity, With<GodotObjRef<KinematicBody2D>>>,
) {
    for SpawnGame(node) in on_spawn_game.iter() {
        let mut parent = commands.spawn();
        parent.insert(GodotObjRef(node.clone())).insert(GameNode);
        for child in children.iter() {
            parent.add_child(child);
        }
    }
}

pub fn end_game(
    mut commands: Commands,
    mut on_end_game: EventReader<EndGame>,
    game: Query<(Entity, &GodotObjRef<Node>), With<GameNode>>,
) {
    for EndGame in on_end_game.iter() {
        for (entity, game_node) in game.iter() {
            let game_node = unsafe { game_node.0.assume_safe() };
            let tree_node = unsafe { game_node.get_tree().unwrap().assume_safe() };

            game_node.queue_free();

            if tree_node.change_scene("res://scenes/GameOver.tscn").is_ok() {
                godot_print!("scene changed!");
            };

            commands.entity(entity).despawn_recursive();
        }
    }
}
