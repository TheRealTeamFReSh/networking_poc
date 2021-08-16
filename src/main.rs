mod network;
mod input;
mod world;

use bevy::prelude::*;
use bevy_backroll::*;
use network::*;

pub struct Materials {
    pub first_player_material: Handle<ColorMaterial>,
    pub second_player_material: Handle<ColorMaterial>,
}

fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        first_player_material: materials.add(Color::rgb(0.898, 0.364, 0.364).into()),
        second_player_material: materials.add(Color::rgb(0.364, 0.580, 0.898).into()),
    });
}

fn start_app(player_num: usize) {
    let bind_addr =  if player_num == 0 {
        "127.0.0.1:4001".parse().unwrap()
    } else {
        "127.0.0.1:4002".parse().unwrap()
    };

    let remote_addr = if player_num == 0 {
        "127.0.0.1:4002".parse().unwrap()
    } else {
        "127.0.0.1:4001".parse().unwrap()
    };

    App::build()
        .add_startup_system(setup_game.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_players.system()))
        .add_plugins(DefaultPlugins)
        .add_plugin(MyBackrollPlugin)
        .insert_resource(StartupNetworkConfig {
            client: player_num,
            bind: bind_addr,
            remote: remote_addr,
        })
        .with_rollback_system::<BackrollConfig, _>(player_movement.system())
        .run();
}

fn main() {
    let mut args = std::env::args();
    let base = args.next().unwrap();

    if let Some(player_num) = args.next() {
        start_app(player_num.parse().unwrap());
    } else {
        let mut child_1 = std::process::Command::new(base.clone())
            .args(&["0"])
            .spawn()
            .unwrap();
        let mut child_2 = std::process::Command::new(base.clone())
            .args(&["1"])
            .spawn()
            .unwrap();

        child_1.wait().unwrap();
        child_2.wait().unwrap();
    }
}