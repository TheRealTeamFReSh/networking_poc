use bevy::prelude::*;

// GameState
#[derive(Clone, PartialEq, Hash)]
#[repr(C)]
pub struct GameState {}

pub fn save_world() -> GameState {
    // println!("Saving the world");
    GameState {}
}

pub fn load_world(_state: In<GameState>) {
    // println!("Loading the world");
}