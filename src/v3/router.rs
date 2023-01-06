#![allow(unreachable_pub, unused)]

use crate::bindings::i_swap_router::*;
use ethers::prelude::{builders::ContractCall, *};
use std::{fmt, sync::Arc};

/// Represents a UniswapV3 SwapRouter.
pub struct Router<M> {
    contract: ISwapRouter<M>,
}

impl<M> Clone for Router<M> {
    fn clone(&self) -> Self {
        Self { contract: self.contract.clone() }
    }
}

impl<M> fmt::Debug for Router<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").field("address", &self.contract.address()).finish()
    }
}

// TODO: Remove
impl<M> std::ops::Deref for Router<M> {
    type Target = ISwapRouter<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}

impl<M> Router<M> {
    /// Returns a reference to the router contract.
    pub fn contract(&self) -> &ISwapRouter<M> {
        &self.contract
    }
}

impl<M: Middleware> Router<M> {
    /// Create a new instance using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = ISwapRouter::new(address, client);
        Self { contract }
    }

    pub fn exact_input(&self, params: ExactInputParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();

        swap_router.exact_input(params)
    }

    pub fn exact_input_single(&self, params: ExactInputSingleParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();

        swap_router.exact_input_single(params)
    }

    pub fn exact_output(&self, params: ExactOutputParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();

        swap_router.exact_output(params)
    }

    pub fn exact_output_single(&self, params: ExactOutputSingleParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();

        swap_router.exact_output_single(params)
    }

    pub fn uniswap_v3_swap_callback(
        &self,
        amount_0_delta: I256,
        amount_1_delta: I256,
        data: Bytes,
    ) -> ContractCall<M, ()> {
        let swap_router = self.contract();

        swap_router.uniswap_v3_swap_callback(amount_0_delta, amount_1_delta, data)
    }
}
