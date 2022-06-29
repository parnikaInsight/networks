pub mod construct{
    use libp2p::{
        identity, PeerId,
    };

    use crate::peer::peer_config;

    #[derive(Debug)]
    pub struct Peer {
        pub local_key: identity::Keypair,
        pub local_peer_id: PeerId,
    }
    
    impl Peer {
        pub fn new() -> Peer {
            let loc_key = peer_config::initializing_peer::create_local_key();
            let local_key = loc_key.clone();
            let local_peer_id = peer_config::initializing_peer::create_local_peer_id(loc_key);
    
            return Peer {
                local_key,
                local_peer_id,
            }
        }
    }
}