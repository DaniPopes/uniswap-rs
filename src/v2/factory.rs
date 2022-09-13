use crate::ProtocolType;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Represents a Uniswap factory.
#[derive(Clone, Copy, Debug)]
pub struct Factory {
    /// The factory address.
    address: Address,

    /// The factory protocol.
    protocol: ProtocolType,
}

impl Factory {
    /// Creates a new instance of Factory from an address.
    pub fn new(address: Address, protocol: ProtocolType) -> Self {
        // assert!(protocol.is_v2(), "protocol must be v2");
        Self { address, protocol }
    }

    /// Creates a new instance of Factory from an address.
    pub fn new_with_chain(chain: Chain, protocol: ProtocolType) -> Option<Self> {
        // assert!(protocol.is_v2(), "protocol must be v2");
        protocol.try_addresses(chain).0.and_then(|address| Some(Self { address, protocol }))
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
    pub const fn pair_codehash(&self) -> H256 {
        self.protocol.get_pair_codehash()
    }

    /// TODO
    pub async fn create_pair<M: Middleware>(
        &self,
        _client: Arc<M>,
    ) -> Result<ContractCall<M, ()>, M> {
        todo!()
    }
}
