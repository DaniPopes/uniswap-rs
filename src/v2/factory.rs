use super::{Library, Pair};
use crate::{bindings::i_uniswap_v2_factory::IUniswapV2Factory, ProtocolType};
use ethers::prelude::*;
use std::sync::Arc;

/// Represents a UniswapV2 factory.
#[derive(Clone, Debug)]
pub struct Factory<M> {
    /// The factory contract.
    contract: IUniswapV2Factory<M>,

    /// The factory protocol.
    protocol: ProtocolType,

    /// The chain.
    chain: Option<Chain>,
}

impl<M> std::ops::Deref for Factory<M> {
    type Target = IUniswapV2Factory<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}

impl<M> Factory<M> {
    /// Returns a reference to the factory contract.
    pub fn contract(&self) -> &IUniswapV2Factory<M> {
        &self.contract
    }

    /// Returns the protocol of the factory.
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }

    /// Returns the deployment code's hash of the pair that this factory deploys.
    ///
    /// Note: `chain` is used only when the pair code hash differs in the same protocol, for example
    /// `Pancakeswap` has two different code hashes for BSC mainnet and testnet.
    pub fn pair_code_hash(&self, chain: Option<Chain>) -> H256 {
        self.protocol.pair_code_hash(chain.or(self.chain))
    }

    /// Sets the factory's chain.
    pub fn set_chain(&mut self, chain: Chain) -> &mut Self {
        self.chain = Some(chain);
        self
    }
}

impl<M: Middleware> Factory<M> {
    /// Creates a new instance using the provided address.
    pub fn new(client: Arc<M>, address: Address, protocol: ProtocolType) -> Self {
        // assert!(protocol.is_v2(), "protocol must be v2");
        let contract = IUniswapV2Factory::new(address, client);
        Self { contract, protocol, chain: None }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        // assert!(protocol.is_v2(), "protocol must be v2");
        if let (Some(address), _) = protocol.try_addresses(chain) {
            let contract = IUniswapV2Factory::new(address, client);
            Some(Self { contract, protocol, chain: Some(chain) })
        } else {
            None
        }
    }

    /// Returns the pair for two token addresses.
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> Pair<M> {
        let address = Library::pair_for(self, token_a, token_b);
        Pair::new(self.client(), address, self.protocol)
    }
}
