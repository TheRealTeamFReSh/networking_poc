use backroll_transport_udp::*;
use bevy_backroll::{backroll::*, *};
use std::net::SocketAddr;
use std::ops::Deref;
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use bevy::{core::FixedTimestep, prelude::*};
use bevy::tasks::IoTaskPool;
use super::{input::*, world::*, Materials};

const MATCH_UPDATE_LABEL: &str = "MATCH_UPDATE";
const DELTA_TIME: f32 = 1.0 / 60.0;

// pub type P2PSession = backroll::P2PSession<BackrollConfig>;

// Backroll config
pub struct BackrollConfig;
impl backroll::Config for BackrollConfig {
    type Input = PlayerInputFrame;
    type State = GameState;
}

// PlayerInput as bytes
bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct PlayerInputFrame: u32 {
        const UP = 1<<0;
        const DOWN = 1<<1;
        const LEFT = 1<<2;
        const RIGHT = 1<<3;
    }
}

// to fix ?
unsafe impl Zeroable for PlayerInputFrame {}
unsafe impl Pod for PlayerInputFrame {}

pub struct StartupNetworkConfig {
    pub client: usize,
    pub bind: SocketAddr,
    pub remote: SocketAddr,
}

#[derive(Clone)]
pub struct Player {
    handle: backroll::PlayerHandle, // the network id
}

pub fn spawn_players(
    mut commands: Commands,
    config: Res<StartupNetworkConfig>,
    pool: Res<IoTaskPool>,
    materials: Res<Materials>,
) {
    let socket = UdpManager::bind(pool.deref().deref().clone(), config.bind).unwrap();
    let peer = socket.connect(UdpConnectionConfig::unbounded(config.remote));

    commands.insert_resource(socket);

    let mut builder = backroll::P2PSession::<BackrollConfig>::build();

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.first_player_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        // make sure to clone the player handles for reference stuff
        .insert(if config.client == 0 {
            // setup local player
            Player {
                handle: builder.add_player(backroll::Player::Local),
            }
        } else {
            // setup remote player
            Player {
                // make sure to clone the peer for reference stuff
                handle: builder.add_player(backroll::Player::Remote(peer.clone())),
            }
        });

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.second_player_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .insert(if config.client == 1 {
                // setup local player
                Player {
                    handle: builder.add_player(backroll::Player::Local),
                }
            } else {
                // setup remote player
                Player {
                    handle: builder.add_player(backroll::Player::Remote(peer)),
                }
            });

        commands.start_backroll_session(builder.start(pool.deref().deref().clone()).unwrap());
}

pub fn player_movement(
    keyboard_input: Res<GameInput<PlayerInputFrame>>,
    mut player_positions: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in player_positions.iter_mut() {
        let input = keyboard_input.get(player.handle).unwrap();
        if input.contains(PlayerInputFrame::LEFT) {
            transform.translation.x -= 2.0;
        }
        if input.contains(PlayerInputFrame::RIGHT) {
            transform.translation.x += 2.0;
        }
        if input.contains(PlayerInputFrame::DOWN) {
            transform.translation.y -= 2.0;
        }
        if input.contains(PlayerInputFrame::UP) {
            transform.translation.y += 2.0;
        }
    }
}

pub struct MyBackrollPlugin;
impl Plugin for MyBackrollPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(BackrollPlugin::<BackrollConfig>::default())
            .with_rollback_run_criteria::<BackrollConfig, _>(
                FixedTimestep::step(DELTA_TIME.into()).with_label(MATCH_UPDATE_LABEL),
            )
            .with_input_sampler_system::<BackrollConfig, _>(sample_input.system())
            .with_world_save_system::<BackrollConfig, _>(save_world.system())
            .with_world_load_system::<BackrollConfig, _>(load_world.system());
    }
}