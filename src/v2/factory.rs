use crate::ProtocolType;
use ethers::prelude::*;
use std::sync::Arc;

/// Represents a Uniswap factory.
#[derive(Debug)]
pub struct Factory<M> {
    client: Arc<M>,

    address: Address,

    protocol: ProtocolType,
}

impl<M> Clone for Factory<M> {
    fn clone(&self) -> Self {
        Self { client: self.client.clone(), address: self.address, protocol: self.protocol }
    }
}

impl<M> From<Factory<M>> for Address {
    fn from(factory: Factory<M>) -> Self {
        factory.address
    }
}

impl<M: Middleware> Factory<M> {
    /// Creates a new instance of Factory from an address.
    pub fn new(client: Arc<M>, address: Address, protocol: ProtocolType) -> Self {
        Self { client, address, protocol }
    }

    /// Creates a new instance of Factory from an address.
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        protocol.try_addresses(chain).0.and_then(|address| Some(Self { client, address, protocol }))
    }

    /// Returns the client.
    pub fn client(&self) -> Arc<M> {
        self.client.clone()
    }

    /// Returns the contract address of the factory.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the protocol of the factory.
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }

    /// Returns the codehash of the pair that this factory deploys.
    pub fn pair_codehash(&self) -> H256 {
        self.protocol.get_pair_codehash()
    }
}
