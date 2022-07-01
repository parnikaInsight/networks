use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, InputStatus, P2PSession, PlayerHandle, SpectatorSession, SyncTestSession};
use std::{hash::Hash, net::SocketAddr};
use env_logger::fmt::Color;
mod game;
mod network;
use crate::game::app_config;
use crate::network::network_config;
use futures::executor::block_on;

async fn async_main() {
    let f1 = app_config::create_app();
    let f2 = network_config::create_network();
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
