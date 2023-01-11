use super::{Library, Pair};
use crate::{contracts::bindings::i_uniswap_v2_factory::IUniswapV2Factory, ProtocolType};
use ethers_core::types::{Address, Chain, H256};
use ethers_providers::Middleware;
use std::sync::Arc;

contract_struct! {
    /// Represents a UniswapV2 factory.
    pub struct Factory<M> {
        /// The factory contract.
        contract: IUniswapV2Factory<M>,

        /// The factory protocol.
        pub protocol: ProtocolType,

        /// The chain.
        pub chain: Option<Chain>,
    }
}

impl<M> Factory<M> {
    /// Returns the protocol of the factory.
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }

    /// Returns the deployment code's hash of the pair that this factory deploys.
    ///
    /// Note: `chain` is used only when the pair code hash differs in the same protocol, for example
    /// `Pancakeswap` has two different code hashes for BSC mainnet and testnet.
    pub fn pair_code_hash(&self, chain: Option<Chain>) -> H256 {
        self.protocol.pair_code_hash(self.chain.or(chain))
    }

    /// Returns the factory's chain.
    pub fn chain(&self) -> Option<Chain> {
        self.chain.clone()
    }

    /// Sets the factory's chain.
    pub fn set_chain(&mut self, chain: Chain) {
        self.chain = Some(chain)
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
        protocol.try_addresses(chain).0.map(|address| {
            let contract = IUniswapV2Factory::new(address, client);
            Self { contract, protocol, chain: Some(chain) }
        })
    }

    /// Returns the pair for two token addresses.
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> Pair<M> {
        let address = Library::pair_for(self, token_a, token_b);
        Pair::new(self.client(), address, self.protocol)
    }
}
