pub mod pair_code_hashes;

mod protocol_type;
pub use protocol_type::*;

use crate::{
    errors::Result,
    v2::{Pair as V2Pair, Protocol as V2Protocol},
    Amount,
};
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, Chain, H256, U256};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

/// An Uniswap V2 or V3 protocol.
///
/// For Universal Router, see [UniversalRouter][crate::universal_router::UniversalRouter].
pub enum Protocol<M> {
    /// A Uniswap V2 protocol.
    V2(V2Protocol<M>),

    /// A Uniswap V3 protocol. Work in progress.
    V3,
}

impl<M> Clone for Protocol<M> {
    fn clone(&self) -> Self {
        match self {
            Self::V2(v2) => Self::V2(v2.clone()),
            Self::V3 => Self::V3,
        }
    }
}

impl<M> fmt::Debug for Protocol<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V2(v2) => f.debug_tuple("V2").field(&v2).finish(),
            Self::V3 => f.pad("V3"),
        }
    }
}

impl<M: Middleware> Protocol<M> {
    /// Creates a new instance using the provided client, factory and router addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        match protocol {
            p if p.is_v2() => Self::V2(V2Protocol::new(client, factory, router, protocol)),
            p if p.is_v3() => todo_v3(),
            _ => unreachable!(),
        }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        match protocol {
            p if p.is_v2() => V2Protocol::new_with_chain(client, chain, protocol).map(Self::V2),
            p if p.is_v3() => todo_v3(),
            _ => unreachable!(),
        }
    }

    /// Returns a pointer to the client.
    #[inline(always)]
    pub fn client(&self) -> Arc<M> {
        match self {
            Self::V2(p) => p.client(),
            Self::V3 => todo_v3(),
        }
    }

    /// Returns a reference to the wrapped [V2Protocol].
    pub fn as_v2(&self) -> Option<&V2Protocol<M>> {
        match self {
            Self::V2(v2) => Some(v2),
            Self::V3 => None,
        }
    }

    /// Returns a mutable reference to the wrapped [V2Protocol].
    pub fn as_v2_mut(&mut self) -> Option<&mut V2Protocol<M>> {
        match self {
            Self::V2(v2) => Some(v2),
            Self::V3 => None,
        }
    }

    /// Returns the wrapped [V2Protocol].
    pub fn into_v2(self) -> Option<V2Protocol<M>> {
        match self {
            Self::V2(v2) => Some(v2),
            Self::V3 => None,
        }
    }

    /// Returns a reference to the wrapped V3Protocol.
    pub fn as_v3(&self) -> Option<()> {
        match self {
            Self::V2(_) => None,
            Self::V3 => Some(()),
        }
    }

    /// Returns a mutable reference to the wrapped V3Protocol.
    pub fn as_v3_mut(&mut self) -> Option<()> {
        match self {
            Self::V2(_) => None,
            Self::V3 => Some(()),
        }
    }

    /// Returns the wrapped V3Protocol.
    pub fn into_v3(self) -> Option<()> {
        match self {
            Self::V2(_) => None,
            Self::V3 => Some(()),
        }
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// The factory's address.
    #[inline(always)]
    pub fn factory_address(&self) -> Address {
        match self {
            Self::V2(p) => p.factory().address(),
            Self::V3 => todo_v3(),
        }
    }

    /// The factory's `pair_codehash` method.
    #[inline(always)]
    pub fn pair_codehash(&self, chain: Option<Chain>) -> H256 {
        match self {
            Self::V2(p) => p.pair_codehash(chain),
            Self::V3 => todo_v3(),
        }
    }

    /// The factory's `create_pair` method.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        match self {
            Self::V2(p) => p.create_pair(token_a, token_b),
            Self::V3 => todo_v3(),
        }
    }

    /// The factory's `pair_for` method.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> V2Pair<M> {
        match self {
            Self::V2(p) => p.pair_for(token_a, token_b),
            Self::V3 => todo_v3(),
        }
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// The router's address.
    #[inline(always)]
    pub fn router_address(&self) -> Address {
        match self {
            Self::V2(p) => p.router().address(),
            Self::V3 => todo_v3(),
        }
    }

    /// The router's `add_liquidity` method.
    #[inline(always)]
    pub fn add_liquidity(
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
            Self::V3 => todo_v3(),
        }
    }

    /// The router's `remove_liquidity` method.
    #[inline(always)]
    pub fn remove_liquidity(
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
            Self::V3 => todo_v3(),
        }
    }

    /// The router's `swap` method.
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
            Self::V3 => todo_v3(),
        }
    }
}

fn todo_v3() -> ! {
    todo!("v3 is not yet implemented")
}
