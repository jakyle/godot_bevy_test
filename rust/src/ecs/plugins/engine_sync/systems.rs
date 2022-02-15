use bevy::prelude::*;
use gdnative::api::*;
use gdnative::prelude::*;

use super::{
    components::{GameNode, GodotObjRef, TitleMenu, PlayingGame},
    events::{SpawnGame, SpawnTitleMenu, DespawnPlayingGame}, resources::GameOver,
};

pub fn spawn_game(
    mut commands: Commands,
    mut on_spawn_game: EventReader<SpawnGame>,
    children: Query<Entity, With<GodotObjRef<KinematicBody2D>>>,
    title_menu: Query<Entity, With<TitleMenu>>
) {
    for SpawnGame(node) in on_spawn_game.iter() {
        let mut parent = commands.spawn();
        parent.insert(GodotObjRef(node.clone())).insert(GameNode).insert(PlayingGame);
        for child in children.iter() {
            parent.add_child(child);
        }

        let entity = title_menu.single();
        commands.entity(entity).despawn();
    }
}

pub fn spawn_title_menu(mut game_over: ResMut<Option<GameOver>>,  mut commands: Commands,  mut on_spawn_title_menu: EventReader<SpawnTitleMenu>) {
    for SpawnTitleMenu(node) in on_spawn_title_menu.iter() {
        let mut parent = commands.spawn();

        let game_status = unsafe {
            node.assume_safe()
                .get_node("GameStatus")
                .unwrap()
                .assume_safe()
                .cast::<Label>()
                .unwrap()
        };

        if let Some(game) = game_over.as_ref(){
            match game {
                GameOver::Win => game_status.set_text("You Win!"),
                GameOver::Lose => game_status.set_text("Oh no you lose :("),
            }
        }

        *game_over = None;

        parent.insert(GodotObjRef(node.clone())).insert(TitleMenu);
    }
}

pub fn end_game(
    game_over: Res<Option<GameOver>>,
    mut send_despawn_playing_game: EventWriter<DespawnPlayingGame>,
    game: Query<&GodotObjRef<Node>, With<GameNode>>,
) {
    if game_over.is_changed() && game_over.is_some() {
        for game_node in game.iter() {
            let game_node = unsafe { game_node.0.assume_safe() };
            let tree_node = unsafe { game_node.get_tree().unwrap().assume_safe() };


            game_node.queue_free();
            send_despawn_playing_game.send(DespawnPlayingGame);

            if tree_node.change_scene("res://scenes/TitleMenu.tscn").is_ok() {
                godot_print!("scene changed!");
            };
        }
    }
}


pub fn despawn_playing_game(
    mut commands: Commands,
    mut on_despawn_playing_game: EventReader<DespawnPlayingGame>,
    playing_game: Query<Entity, With<PlayingGame>>
) {
    for DespawnPlayingGame in on_despawn_playing_game.iter() {
        for entity in playing_game.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}