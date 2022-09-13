use super::{Factory, Router};
use crate::{
    errors::{FactoryError, RouterError},
    Amount, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// TODO
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
    /// TODO
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let factory = Factory::new(factory, protocol);
        let router = Router::new(router, protocol);
        Self { client, factory, router }
    }

    /// TODO
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

    /// Returns the factory.
    pub fn factory(&self) -> Factory {
        self.factory
    }

    /// Returns the router.
    pub fn router(&self) -> Router {
        self.router
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// TODO
    #[inline(always)]
    pub const fn pair_codehash(&self) -> H256 {
        self.factory.pair_codehash()
    }

    /// TODO
    #[inline(always)]
    pub async fn create_pair(&self) -> Result<ContractCall<M, ()>, FactoryError<M>> {
        todo!("create_pair is not yet implemented")
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// TODO
    #[inline(always)]
    pub async fn add_liquidity(&self) -> Result<ContractCall<M, Vec<U256>>, RouterError<M>> {
        todo!("add_liquidity is not yet implemented")
    }

    /// TODO
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, Vec<U256>>, RouterError<M>> {
        self.router
            .swap(self.client.clone(), self.factory, amount, slippage_tolerance, path, to, deadline)
            .await
    }
}
