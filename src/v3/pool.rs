use crate::{bindings::i_uniswap_v3_pool::IUniswapV3Pool, ProtocolType};
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, Chain, H256};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

type Tokens = (Address, Address);
type Reserves = (u128, u128, u32);

contract_struct! {
    /// Represents a UniswapV3 liquidity pool, composed of 2 different ERC20 tokens.
    pub struct Pool<M> {
        /// The pool contract.
        contract: IUniswapV3Pool<M>,

        /// The ordered tokens of the pool.
        tokens: Option<Tokens>,

        /// Whether the pool is currently deployed in the client's network.
        deployed: bool,

        /// The token reserves of the pool.
        reserves: Option<Reserves>,

        /// The protocol of the pool.
        pub protocol: ProtocolType,
    }
}

impl<M> fmt::Display for Pool<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address = self.address();
        if self.tokens.is_none() && self.reserves.is_none() {
            return writeln!(f, "Pool: {address:?}")
        }
        writeln!(f, "Pool:     {address:?}")?;
        if let Some((a, b)) = self.tokens {
            writeln!(f, "Token0:   {a:?}")?;
            write!(f, "Token1:   {b:?}")?;
            if self.reserves.is_some() {
                writeln!(f)?
            };
        }
        if let Some((a, b, _)) = self.reserves {
            writeln!(f, "Reserve0: {a:?}")?;
            write!(f, "Reserve1: {b:?}")?;
        }
        Ok(())
    }
}

// TODO: Remove
impl<M> std::ops::Deref for Pool<M> {
    type Target = IUniswapV3Pool<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}

impl<M> Pool<M> {
    /// Returns a reference to the pool contract.
    pub fn contract(&self) -> &IUniswapV3Pool<M> {
        &self.contract
    }

    /// Returns whether the pool has been deployed.
    ///
    /// Note: this will always be false before syncing.
    pub fn deployed(&self) -> bool {
        self.deployed
    }

    /// Returns the addresses of the tokens that make up this pool.
    ///
    /// Note: this will always be None before syncing.
    pub fn tokens(&self) -> Option<Tokens> {
        self.tokens
    }

    /// Returns the reserves of the pool.
    ///
    /// Note: this will always be None before syncing.
    pub fn reserves(&self) -> Option<Reserves> {
        self.reserves
    }

    /// Returns the hash of the pool's deployment code. This can be used to determinalistically
    /// calculate the address of the pool given the addresses of 2 (sorted) tokens.
    ///
    /// Note: `chain` is used only when the pool code hash differs in the same protocol, for example
    /// `Pancakeswap` has two different code hashes for BSC mainnet and testnet.
    pub fn code_hash(&self, chain: Option<Chain>) -> H256 {
        self.protocol.pair_code_hash(chain)
    }
}

impl<M: Middleware> Pool<M> {
    /// Creates a new instance using the provided client and address.
    pub fn new(client: Arc<M>, address: Address, protocol: ProtocolType) -> Self {
        let contract = IUniswapV3Pool::new(address, client);
        Self { contract, tokens: None, deployed: false, reserves: None, protocol }
    }

    // /// Creates a new instance using the provided client, factory and tokens' addresses.
    // pub fn new_with_factory(
    //     factory: &Factory<M>,
    //     token0: Address,
    //     token1: Address,
    // ) -> Result<Self> {
    //     let (token0, token1) = Library::sort_tokens(token0, token1);
    //     let address = Library::pool_for(factory, token0, token1);
    //     let contract = IUniswapV3Pool::new(address, factory.client());

    //     Ok(Self {
    //         contract,
    //         tokens: Some((token0, token1)),
    //         deployed: false,
    //         reserves: None,
    //         protocol: factory.protocol(),
    //     })
    // }

    /// Returns the contract calls for getting the addresses of the pool's tokens.
    pub fn get_tokens(&self) -> (ContractCall<M, Address>, ContractCall<M, Address>) {
        (self.token_0(), self.token_1())
    }
}
