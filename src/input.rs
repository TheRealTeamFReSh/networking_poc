use bevy::prelude::*;
use bevy_backroll::backroll::PlayerHandle;
use super::network::*;

pub fn sample_input(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> PlayerInputFrame {
    let mut local_input = PlayerInputFrame::empty();

    // local input handling
    if keyboard_input.pressed(KeyCode::Left) {
        local_input.insert(PlayerInputFrame::LEFT);
        println!("Local: LEFT");
    } else if keyboard_input.pressed(KeyCode::Right) {
        local_input.insert(PlayerInputFrame::RIGHT);
        println!("Local: Right");
    }

    if keyboard_input.pressed(KeyCode::Up) {
        local_input.insert(PlayerInputFrame::UP);
        println!("Local: UP");
    } else if keyboard_input.pressed(KeyCode::Down) {
        local_input.insert(PlayerInputFrame::DOWN);
        println!("Local: Down");
    }

    local_input
}