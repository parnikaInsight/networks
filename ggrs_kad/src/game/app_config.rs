use bevy::prelude::*;
use futures::StreamExt;
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;
use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_ggrs::*;
use ggrs::PlayerType;

pub async fn create_app(){
    print!("in app");
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_plugins(DefaultPlugins)
        .run();
}
