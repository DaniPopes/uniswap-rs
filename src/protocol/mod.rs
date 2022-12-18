use crate::{
    errors::Result,
    v2::{Factory as V2Factory, Pair as V2Pair, Protocol as V2Protocol, Router as V2Router},
    Amount,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

pub mod pair_code_hashes;

mod protocol_type;
pub use protocol_type::*;

/// Represents an automated market maker, a protocol that facilitates peer-to-peer market making and
/// swapping of ERC-20 tokens on the Ethereum blockchain.
#[derive(Clone, Debug)]
pub enum Protocol<M> {
    /// The UniswapV2 protocol.
    V2(V2Protocol<M>),

    /// The UniswapV3 protocol. WIP.
    V3,
}

impl<M: Middleware> Protocol<M> {
    /// Creates a new instance using the provided client, factory and router addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        match protocol {
            p if p.is_v2() => Self::V2(V2Protocol::new(client, factory, router, protocol)),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol \"{p:?}\" is neither v2 nor v3"),
        }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        match protocol {
            p if p.is_v2() => V2Protocol::new_with_chain(client, chain, protocol).map(Self::V2),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol \"{p:?}\" is neither v2 nor v3"),
        }
    }

    /// Returns a pointer to the client.
    #[inline(always)]
    pub fn client(&self) -> Arc<M> {
        match self {
            Self::V2(p) => p.client(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// Returns a reference to the factory.
    #[inline(always)]
    pub fn factory(&self) -> &V2Factory<M> {
        match self {
            Self::V2(p) => p.factory(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `pair_codehash` method.
    #[inline(always)]
    pub fn pair_codehash(&self, chain: Option<Chain>) -> H256 {
        match self {
            Self::V2(p) => p.pair_codehash(chain),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `create_pair` method.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        match self {
            Self::V2(p) => p.create_pair(token_a, token_b),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `pair_for` method.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> Result<V2Pair<M>> {
        match self {
            Self::V2(p) => p.pair_for(token_a, token_b),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns a reference to the router.
    #[inline(always)]
    pub fn router(&self) -> &V2Router<M> {
        match self {
            Self::V2(p) => p.router(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `add_liquidity` method.
    #[inline(always)]
    pub async fn add_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256, U256)>> {
        match self {
            Self::V2(p) => p.add_liquidity(
                token_a,
                token_b,
                amount_a_desired,
                amount_b_desired,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            ),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `remove_liquidity` method.
    #[inline(always)]
    pub async fn remove_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256)>> {
        match self {
            Self::V2(p) => p.remove_liquidity(
                token_a,
                token_b,
                liquidity,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            ),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `swap` method.
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: &[Address],
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> Result<ContractCall<M, Vec<U256>>> {
        match self {
            Self::V2(p) => p.swap(amount, slippage_tolerance, path, to, deadline, weth).await,
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }
}
