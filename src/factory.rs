use crate::Protocol;
use ethers::prelude::*;

/// Represents a Uniswap factory.
#[derive(Clone, Copy, Debug)]
pub struct Factory {
    address: Address,
    protocol: Protocol,
}

impl From<Factory> for Address {
    fn from(factory: Factory) -> Self {
        factory.address
    }
}

impl Factory {
    /// Creates a new instance of a Uniswap factory.
    /// Note: either an address or a chain must be provided.
    pub fn new(address: Option<Address>, chain: Option<Chain>, protocol: Protocol) -> Self {
        let address = address.unwrap_or_else(|| {
            protocol
                .addresses(
                    chain.expect("Factory::new: Must provide at least one of: address or chain"),
                )
                .1
        });

        Self { address, protocol }
    }

    /// Returns the contract address of the factory.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the protocol of the factory.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Returns the codehash of the pair that this factory deploys.
    pub fn pair_codehash(&self) -> H256 {
        self.protocol.get_pair_codehash()
    }
}
