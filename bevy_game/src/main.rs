use bevy::{app::AppExit, prelude::*};
use bevy_ggrs::{GGRSPlugin, SessionType};
use ggrs::{Config, InputStatus, P2PSession, PlayerHandle, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use std::{hash::Hash, net::SocketAddr};
use bytemuck::{Pod, Zeroable};
use structopt::StructOpt;
//use bevy_ggrs::{Rollback, RollbackIdProvider};

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";

const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

const MOVEMENT_SPEED: f32 = 0.005;
const MAX_SPEED: f32 = 0.05;
const FRICTION: f32 = 0.9;
const PLANE_SIZE: f32 = 5.0;
const CUBE_SIZE: f32 = 0.2;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

struct NetworkStatsTimer(Timer);

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = BoxInput;
    type State = u8;
    type Address = SocketAddr;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8,
}

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

// Components that should be saved/loaded need to implement the `Reflect` trait
#[derive(Default, Reflect, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// You can also register resources. If your Component / Resource implements Hash, you can make use of `#[reflect(Hash)]`
// in order to allow a GGRS `SyncTestSession` to construct a checksum for a world snapshot
#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    BoxInput { inp: input }
}

pub fn setup_system(
    mut commands: Commands,
  //  mut rip: ResMut<RollbackIdProvider>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    p2p_session: Option<Res<P2PSession<GGRSConfig>>>,
) {
    let num_players = p2p_session
        .map(|s| s.num_players())
        .expect("No GGRS session found");

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // player cube - just spawn whatever entity you want, then add a `Rollback` component with a unique id (for example through the `RollbackIdProvider` resource).
    // Every entity that you want to be saved/loaded needs a `Rollback` component with a unique rollback id.
    // When loading entities from the past, this extra id is necessary to connect entities over different game states
    let r = PLANE_SIZE / 4.;

    for handle in 0..num_players {
        let rot = handle as f32 / num_players as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let z = r * rot.sin();

        let mut transform = Transform::default();
        transform.translation.x = x;
        transform.translation.y = CUBE_SIZE / 2.;
        transform.translation.z = z;

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE })),
                material: materials.add(PLAYER_COLORS[handle as usize].into()),
                transform,
                ..Default::default()
            })
            .insert(Player { handle })
            .insert(Velocity::default());
            // this component indicates bevy_GGRS that parts of this entity should be saved and loaded
           // .insert(Rollback::new(rip.next_id()));
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 7.5, 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

#[allow(dead_code)]
pub fn increase_frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub fn move_cube_system(
    mut query: Query<(&mut Transform, &mut Velocity, &Player)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
) {
    for (mut t, mut v, p) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;
        // set velocity through key presses
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            v.z -= MOVEMENT_SPEED;
        }
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            v.z += MOVEMENT_SPEED;
        }
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            v.x -= MOVEMENT_SPEED;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            v.x += MOVEMENT_SPEED;
        }

        // slow down
        if input & INPUT_UP == 0 && input & INPUT_DOWN == 0 {
            v.z *= FRICTION;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT == 0 {
            v.x *= FRICTION;
        }
        v.y *= FRICTION;

        // constrain velocity
        let mag = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if mag > MAX_SPEED {
            let factor = MAX_SPEED / mag;
            v.x *= factor;
            v.y *= factor;
            v.z *= factor;
        }

        // apply velocity
        t.translation.x += v.x;
        t.translation.y += v.y;
        t.translation.z += v.z;

        // constrain cube to plane
        t.translation.x = t.translation.x.max(-1. * (PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.x = t.translation.x.min((PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.z = t.translation.z.max(-1. * (PLANE_SIZE - CUBE_SIZE) * 0.5);
        t.translation.z = t.translation.z.min((PLANE_SIZE - CUBE_SIZE) * 0.5);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read cmd line arguments
    let opt = Opt::from_args();
    let num_players = opt.players.len();
    print!("Num Players: {}", num_players);
    assert!(num_players > 0);

    // create a GGRS session
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(12) // (optional) set max prediction window
        .with_input_delay(2); // (optional) set input delay for the local player
    
    
    for (i, player_addr) in opt.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            sess_build = sess_build.add_player(PlayerType::Local, i)?;
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse()?;
            sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), i)?;
        }
    }

    // start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port(opt.local_port)?;
    let sess = sess_build.start_p2p_session(socket)?;

    let mut app = App::new();
    // app.insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
    //     .add_plugins(DefaultPlugins) 
    //     .run();

    
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 720.,
            height: 720.,
            title: "GGRS Box Game".to_owned(),
            ..Default::default()
        })
        .insert_resource(opt)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        // add your GGRS session
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession)
        // register a resource that will be rolled back
        //.insert_resource(FrameCount { frame: 0 })
        //print some network stats - not part of the rollback schedule as it does not need to be rolled back
        .insert_resource(NetworkStatsTimer(Timer::from_seconds(2.0, true)))
        .add_system(print_network_stats_system)
        .add_system(print_events_system)
        .run();

    Ok(()) //solves expected Result<(), Box dyn err>
}

fn print_events_system(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        println!("GGRS Event: {:?}", event);
    }
}

fn print_network_stats_system(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Option<Res<P2PSession<GGRSConfig>>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(sess) = p2p_session {
            let num_players = sess.num_players() as usize;
            for i in 0..num_players {
                if let Ok(stats) = sess.network_stats(i) {
                    println!("NetworkStats for player {}: {:?}", i, stats);
                }
            }
        }
    }
}
