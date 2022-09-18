use super::{Factory, Router};
use crate::{errors::RouterError, Amount, ProtocolType};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Represents the UniswapV2 protocol.
#[derive(Debug)]
pub struct V2Protocol<M> {
    /// The client.
    client: Arc<M>,

    /// The liquidity pair factory.
    factory: Factory,

    /// The router.
    router: Router,
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
    pub const fn pair_codehash(&self) -> Option<H256> {
        self.factory.pair_code_hash()
    }

    /// The factory's create_pair method. See documentation of [Factory] for more details.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        self.factory.create_pair(self.client.clone(), token_a, token_b)
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns the router.
    pub fn router(&self) -> Router {
        self.router
    }

    /// The router's add_liquidity method. See documentation of [Router] for more details.
    #[inline(always)]
    pub async fn add_liquidity(&self) -> Result<ContractCall<M, Vec<U256>>, RouterError<M>> {
        todo!("add_liquidity is not yet implemented")
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
