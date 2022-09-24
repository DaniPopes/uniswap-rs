use super::{Factory, Pair, Router};
use crate::{
    errors::{FactoryResult, RouterResult},
    Amount, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Represents the UniswapV2 protocol.
#[derive(Clone, Debug)]
pub struct V2Protocol<M> {
    /// The liquidity pair factory.
    factory: Factory<M>,

    /// The swap router.
    router: Router<M>,
}

impl<M: Middleware> V2Protocol<M> {
    /// Creates a new instance using the provided client, factory and tokens' addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let factory = Factory::new(client.clone(), factory, protocol);
        let router = Router::new(client, router, protocol);
        Self { factory, router }
    }

    /// Creates a new instance using the provided client and chain.
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Self {
        let (factory, router) = protocol.addresses(chain);
        let factory = Factory::new(client.clone(), factory, protocol);
        let router = Router::new(client, router, protocol);
        Self { factory, router }
    }

    /// Returns a pointer to the client.
    pub fn client(&self) -> Arc<M> {
        self.factory.client()
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// Returns the factory.
    pub fn factory(&self) -> &Factory<M> {
        &self.factory
    }

    /// The factory's `pair_codehash` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn pair_codehash(&self, chain: Option<Chain>) -> H256 {
        self.factory.pair_code_hash(chain)
    }

    /// The factory's `create_pair` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        self.factory.create_pair(token_a, token_b)
    }

    /// The factory's `pair_for` method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> FactoryResult<Pair<M>, M> {
        self.factory.pair_for(token_a, token_b)
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns the router.
    pub fn router(&self) -> &Router<M> {
        &self.router
    }

    /// The router's `add_liquidity` method. See documentation of [Router] for more details.
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
    ) -> RouterResult<ContractCall<M, (U256, U256, U256)>, M> {
        self.router.add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )
    }

    /// The router's `remove_liquidity` method. See documentation of [Router] for more details.
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
    ) -> RouterResult<ContractCall<M, (U256, U256)>, M> {
        self.router.remove_liquidity(
            token_a,
            token_b,
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )
    }

    /// The router's `swap` method. See documentation of [Router] for more details.
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: &[Address],
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> RouterResult<ContractCall<M, Vec<U256>>, M> {
        self.router.swap(&self.factory, amount, slippage_tolerance, path, to, deadline, weth).await
    }
}
