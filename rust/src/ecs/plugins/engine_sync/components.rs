use bevy::prelude::*;
use gdnative::prelude::*;

#[derive(Component)]
pub struct GodotObjRef<T: GodotObject>(pub Ref<T>);

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct TitleMenu;

#[derive(Component)]
pub struct PlayingGame;