use async_std::{io, task};
use futures::{prelude::*, select};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, Kademlia, KademliaConfig,
    Quorum, Record,
};
use libp2p::{
    development_transport, Swarm, mdns::{Mdns, MdnsConfig}, swarm::{SwarmEvent},
};
use std::{error::Error, time::Duration};

mod peer;
use peer::network_config::network::MyBehaviour;
use peer::peer_construct::construct::Peer;
use peer::find_my_peers::finding;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let peer = Peer::new();

    println!("peer is {:?}", peer);
    
    let transport = development_transport(peer.local_key).await?;
    
    //Create a swarm to manage peers and events.
    let mut swarm = {
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(peer.local_peer_id);
        let kademlia = Kademlia::with_config(peer.local_peer_id, store, cfg);

        // Create a Kademlia behaviour.
        //let store = MemoryStore::new(peer.local_peer_id);
        //let kademlia = Kademlia::new(peer.local_peer_id, store);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let behaviour = MyBehaviour { kademlia, mdns };
        Swarm::new(transport, behaviour, peer.local_peer_id)
    };

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("listening on ports");

    // Kick it off.
    loop {
        select! {
            line = stdin.select_next_some() => handle_input_line(line.expect("Stdin not to close"), &mut swarm),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {:?}", address);
                },
                _ => {}
            }
        }
        println!("looping");
    }
}

fn handle_input_line(line: String, swarm: &mut Swarm<MyBehaviour>) {

    let kademlia: &mut Kademlia<MemoryStore> = &mut swarm.behaviour_mut().kademlia;

    println!("handle_input_line");
    let mut args = line.split(' ');

    match args.next() {
        /*Some("FIND") => {
            let other_event = swarm.select_next_some().await;
            let id = {
                match args.next() {
                    Some(id) => {
                        finding::search(String::from(id), swarm);
                        return;
                    },
                    None => {
                        eprintln!("Expected peerID");
                        return;
                    }
                }
            };
        }*/
        Some("LIST_PEERS") => {
            finding::list_peers(swarm);
        }

        Some("GET") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_record(key, Quorum::One);
        }
        Some("GET_PROVIDERS") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_providers(key);
        }
        Some("PUT") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            let value = {
                match args.next() {
                    Some(value) => value.as_bytes().to_vec(),
                    None => {
                        eprintln!("Expected value");
                        return;
                    }
                }
            };
            let record = Record {
                key,
                value,
                publisher: None,
                expires: None,
            };
            kademlia
                .put_record(record, Quorum::One)
                .expect("Failed to store record locally.");
        }
        Some("PUT_PROVIDER") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            kademlia
                .start_providing(key)
                .expect("Failed to start providing key");
        }
        _ => {
            eprintln!("expected GET, GET_PROVIDERS, PUT or PUT_PROVIDER");
        }
    }
}



