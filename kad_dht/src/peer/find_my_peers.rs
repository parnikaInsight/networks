pub mod finding{
    use libp2p::{Swarm,};
    use libp2p::kad::record::store::MemoryStore;
    use libp2p::kad::{GetClosestPeersOk, GetClosestPeersError, Kademlia,};
    use crate::peer::network_config::network::MyBehaviour;

    //get peers
    pub fn list_peers(swarm: &mut Swarm<MyBehaviour>) {
        let kademlia: &mut Kademlia<MemoryStore> = &mut swarm.behaviour_mut().kademlia;
        for bucket in kademlia.kbuckets() {
            if bucket.num_entries() > 0 {
                for item in bucket.iter() {
                    println!("Peer ID: {:?}", item.node.key);
                }
            }
        }
    }

    // Order Kademlia to search for closest peer.
    pub fn search(result: Result<GetClosestPeersOk, GetClosestPeersError>) {
        {
            match result {
                Ok(ok) => {
                    if !ok.peers.is_empty() {
                        println!("Query finished with closest peers: {:#?}", ok.peers)
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query finished with no closest peers.")
                    }
                }
                Err(GetClosestPeersError::Timeout { peers, .. }) => {
                    if !peers.is_empty() {
                        println!("Query timed out with closest peers: {:#?}", peers)
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query timed out with no closest peers.");
                    }
                }
            };
        }
    }
}