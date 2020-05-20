//! # Light Client Supervisor
//!
//! A library for supervising the light client processes and detecting forks. This should be a
//! primary gateway for external users wanting to securily verify a Tendermint blockchain.

use tendermint::lite::types::Height;
// use tendermint::block::signed_header::SignedHeader;
// use std::time::SystemTime;

#[derive(Debug)]
struct LightBlock {
    height: Height,
}

impl PartialEq for LightBlock {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

trait LightClient {
    // lifecycle management
    fn run(&self);
    fn shutdown(&self);

    // fork detection
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
    pub fn new(
        trust_n: u8,
        light_clients: Vec<Box<dyn LightClient>>,
    ) -> Self {
        // At least two light clients are required.
        if trust_n < 2 {
            panic!("trust number must be greater than two")
        }
        if light_clients.len() < trust_n.into() {
            panic!("number of light clients is less than the trust number")
        }
        Self {
            trust_n: trust_n,
            light_clients: light_clients,
        }
    }

    pub fn run(&self) {
        for lc in &self.light_clients {
            lc.run()
        }
    }

    pub fn shutdown(&self) {
        for lc in &self.light_clients {
            lc.shutdown()
        }
    }

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
    use super::*;

    struct MockLightClient {
        // blocks: Vec<LightBlock>,
    }

    impl LightClient for MockLightClient {
        fn run(&self) {

        }

        fn shutdown(&self) {

        }

        fn light_block(&self, _h: Height) -> Option<LightBlock> {
            None
        }
    }

    #[test]
    fn supervisor_detects_forks() {
        let block = LightBlock{height: 1};
        let clients: Vec<Box<dyn LightClient>> = vec![Box::new(MockLightClient{}), Box::new(MockLightClient{})];
        let sup = Supervisor::new(2, clients);

        sup.run();

        assert_eq!(block, sup.light_block(1).unwrap())
    }
}
