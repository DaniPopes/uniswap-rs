use crate::{
    bindings::i_uniswap_v2_pair::IUniswapV2Pair, factory::Factory, UniswapV2Library,
    UniswapV2LibraryError,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

type Tokens = (Address, Address);
type Reserves = (u128, u128, u32);

/// Represents a UniswapV2 liquidity pair, composed of 2 different tokens.
#[derive(Clone)]
pub struct Pair<M> {
    /// The client.
    client: Arc<M>,

    /// The pair address. Might not be currently deployed.
    address: Address,

    /// The tokens of the pair.
    tokens: Option<Tokens>,

    /// Whether the pair is currently deployed in the client's network.
    deployed: bool,

    /// The token reserves of the pair.
    reserves: Reserves,
}

// Skip client in formatting
impl<M> std::fmt::Debug for Pair<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pair")
            .field("address", &self.address)
            .field("tokens", &self.tokens)
            .field("deployed", &self.deployed)
            .field("reserves", &self.reserves)
            .finish()
    }
}

impl<M: Middleware> Pair<M> {
    pub async fn new(
        client: Arc<M>,
        factory: Factory,
        tokens: Option<Tokens>,
        address: Option<Address>,
    ) -> Result<Self, UniswapV2LibraryError> {
        let address = if let Some(address) = address {
            address
        } else {
            let tokens =
                tokens.expect("Must provide either a pair address or the two token addresses");
            UniswapV2Library::pair_for(factory, tokens.0, tokens.1)?
        };

        Ok(Self { client, address, tokens, deployed: false, reserves: (0, 0, 0) })
    }

    /// Manually set whether the pair has been deployed.
    pub fn deployed(&mut self, deployed: bool) -> &mut Self {
        self.deployed = deployed;
        self
    }

    /// Manually set the reserves of the pair.
    pub fn reserves(&mut self, reserves: Reserves) -> &mut Self {
        self.reserves = reserves;
        self
    }

    /// Returns the contract calls for getting the addresses of the pair's tokens.
    pub fn get_tokens(&self) -> (ContractCall<M, Address>, ContractCall<M, Address>) {
        let pair = IUniswapV2Pair::new(self.address, self.client.clone());
        (pair.token_0(), pair.token_1())
    }

    /// Returns the contract call for getting the reserves of the pair.
    pub fn get_reserves(&self) -> ContractCall<M, Reserves> {
        IUniswapV2Pair::new(self.address, self.client.clone()).get_reserves()
    }

    /// Set the reserves and deployed status of the pair by querying the blockchain.
    pub async fn set_reserves(&mut self) -> Result<&mut Self, ContractError<M>> {
        let call = self.get_reserves();
        let (deployed, reserves) = match call.call().await {
            Ok(reserves) => (true, reserves),
            Err(e) => {
                match e {
                    // Contract has not yet been deployed
                    ContractError::ContractNotDeployed => (false, (0, 0, 0)),
                    e => return Err(e),
                }
            }
        };

        self.deployed = deployed;
        self.reserves = reserves;

        Ok(self)
    }
}
