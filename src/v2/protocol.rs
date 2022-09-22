use super::{Factory, Pair, Router};
use crate::{
    errors::{FactoryResult, RouterError},
    Amount, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::{fmt, sync::Arc};

/// Represents the UniswapV2 protocol.
pub struct V2Protocol<M> {
    /// The client.
    client: Arc<M>,

    /// The liquidity pair factory.
    factory: Factory,

    /// The router.
    router: Router,
}

// Skip client in formatting
impl<M> fmt::Debug for V2Protocol<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("V2Protocol")
            .field("factory", &self.factory)
            .field("router", &self.router)
            .finish()
    }
}

impl<M> Clone for V2Protocol<M> {
    fn clone(&self) -> Self {
        Self { client: self.client.clone(), factory: self.factory, router: self.router }
    }
}

impl<M: Middleware> V2Protocol<M> {
    /// Creates a new instance using the provided client, factory and tokens' addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let factory = Factory::new(factory, protocol);
        let router = Router::new(router, protocol);
        Self { client, factory, router }
    }

    /// Creates a new instance using the provided client and chain.
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Self {
        let (factory, router) = protocol.addresses(chain);
        let factory = Factory::new(factory, protocol);
        let router = Router::new(router, protocol);
        Self { client, factory, router }
    }

    /// Returns the client.
    pub fn client(&self) -> Arc<M> {
        self.client.clone()
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// Returns the factory.
    pub fn factory(&self) -> Factory {
        self.factory
    }

    /// The factory's pair_codehash method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub const fn pair_codehash(&self) -> H256 {
        self.factory.pair_code_hash()
    }

    /// The factory's create_pair method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        self.factory.create_pair(self.client.clone(), token_a, token_b)
    }

    /// The factory's pair_for method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> FactoryResult<Pair<M>, M> {
        self.factory.pair_for(self.client.clone(), token_a, token_b)
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns the router.
    pub fn router(&self) -> Router {
        self.router
    }

    /// The router's add_liquidity method. See documentation of [Router] for more details.
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
    ) -> Result<ContractCall<M, (U256, U256, U256)>, RouterError<M>> {
        self.router.add_liquidity(
            self.client.clone(),
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

    /// The router's remove_liquidity method. See documentation of [Router] for more details.
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
    ) -> Result<ContractCall<M, (U256, U256)>, RouterError<M>> {
        self.router.remove_liquidity(
            self.client.clone(),
            token_a,
            token_b,
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )
    }

    /// The router's swap method. See documentation of [Router] for more details.
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> Result<ContractCall<M, Vec<U256>>, RouterError<M>> {
        self.router
            .swap(
                self.client.clone(),
                self.factory,
                amount,
                slippage_tolerance,
                path,
                to,
                deadline,
                weth,
            )
            .await
    }
}
