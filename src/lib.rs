//! # Light Client Supervisor
//!
//! A library for supervising the light client processes and detecting forks. This should be a
//! primary gateway for external users wanting to securily verify a Tendermint blockchain.

pub use tendermint::lite::types::Height;
pub use tendermint::block::signed_header::SignedHeader;
pub use std::time::SystemTime;

struct LightBlock {
    height: Height,
    header: SignedHeader,
}

trait LightClient {
    fn light_block(&self, h: Height) -> Option<LightBlock>;
}

// A supervisor for light clients, which makes sure they are up and running and performs fork
// detection.
struct Supervisor {
    // minimum number of light clients after which we trust a block
    trust_n: u8,
    // a set of light clients
    light_clients: Vec<Box<dyn LightClient>>,
}

impl Supervisor {
    // At least two light clients are required.
    pub fn new(
        trust_n: u8,
        light_clients: Vec<Box<dyn LightClient>>,
    ) -> Self {
        if trust_n < 2 {

        }
        if light_clients.len() < trust_n.into() {
            panic!("number of light clients is less than the trust number")
        }
        Self {
            trust_n: trust_n,
            light_clients: light_clients,
        }
    }

    // pub fn run(&self) {
    //     for lc in self.light_clients {
    //         lc.run()
    //     }
    // }

    pub fn light_block(&self, h: Height) -> Option<LightBlock> {
        let mut i = self.trust_n;

        while i > 0 {
            // match self.light_clients.light_block(h) {
            //     Some(b) => 0,
            //         None => continue,
            // }
            i-=1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
