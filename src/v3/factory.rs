use crate::{bindings::i_uniswap_v3_factory::IUniswapV3Factory, ProtocolType};
use ethers::prelude::*;
use std::sync::Arc;

/// Represents a UniswapV3 factory.
#[derive(Clone, Debug)]
pub struct Factory<M> {
    /// The factory contract.
    contract: IUniswapV3Factory<M>,

    /// The factory protocol.
    pub protocol: ProtocolType,

    /// The chain.
    pub chain: Option<Chain>,
}

impl<M> std::ops::Deref for Factory<M> {
    type Target = IUniswapV3Factory<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}

impl<M> Factory<M> {
    /// Returns a reference to the factory contract.
    pub fn contract(&self) -> &IUniswapV3Factory<M> {
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
        self.protocol.pair_code_hash(self.chain.or(chain))
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
        let contract = IUniswapV3Factory::new(address, client);
        Self { contract, protocol, chain: None }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        protocol.try_addresses(chain).0.map(|address| {
            let contract = IUniswapV3Factory::new(address, client);
            Self { contract, protocol, chain: Some(chain) }
        })
    }

    /// Sets the contract's address.
    pub fn set_address(&mut self, address: impl Into<Address>)
    where
        M: Clone,
    {
        self.contract = self.contract.at(address).into();
    }

    // /// Returns the pair for two token addresses.
    // pub fn pair_for(&self, token_a: Address, token_b: Address) -> Pair<M> {
    //     let address = Library::pair_for(self, token_a, token_b);
    //     Pair::new(self.client(), address, self.protocol)
    // }
}
