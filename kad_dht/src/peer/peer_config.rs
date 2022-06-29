pub mod initializing_peer{
    use libp2p::{
        identity, PeerId, 
    };

    pub fn create_local_key() -> identity::Keypair {
        let local_key: identity::Keypair = identity::Keypair::generate_ed25519();
        return local_key;
    }

    pub fn create_local_peer_id(local_key: identity::Keypair) -> PeerId {
        let local_peer_id: PeerId = PeerId::from(local_key.public());
        return local_peer_id;
    }
}

