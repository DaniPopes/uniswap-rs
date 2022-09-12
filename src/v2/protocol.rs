use crate::ProtocolType;
use ethers::prelude::*;

use super::{Factory, Router};
use std::sync::Arc;

/// TODO
#[derive(Debug)]
pub struct V2Protocol<M> {
    client: Arc<M>,

    factory: Factory<M>,

    router: Router<M>,
}

impl<M> Clone for V2Protocol<M> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            factory: self.factory.clone(),
            router: self.router.clone(),
        }
    }
}

impl<M: Middleware> V2Protocol<M> {
    /// TODO
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let factory = Factory::new(client.clone(), factory, protocol);
        let router = Router::new(client.clone(), router);
        Self { client, factory, router }
    }

    /// TODO
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Self {
        let (factory, router) = protocol.addresses(chain);
        let factory = Factory::new(client.clone(), factory, protocol);
        let router = Router::new(client.clone(), router);
        Self { client, factory, router }
    }
}
