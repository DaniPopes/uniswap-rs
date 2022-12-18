#![allow(unreachable_pub, unused)]

use crate::bindings::i_swap_router::*;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Represents a UniswapV3 SwapRouter.
#[derive(Clone, Debug)]
pub struct Router<M>(ISwapRouter<M>);

impl<M> std::ops::Deref for Router<M> {
    type Target = ISwapRouter<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}

impl<M> Router<M> {
    /// Returns a reference to the SwapRouter contract
    pub fn contract(&self) -> &ISwapRouter<M> {
        &self.0
    }
}

impl<M: Middleware> Router<M> {
    /// Create a new instance using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = ISwapRouter::new(address, client);
        Self(contract)
    }

    pub fn exact_input(&self, params: ExactInputParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();
        let call = swap_router.exact_input(params);
        call
    }

    pub fn exact_input_single(&self, params: ExactInputSingleParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();
        let call = swap_router.exact_input_single(params);
        call
    }

    pub fn exact_output(&self, params: ExactOutputParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();
        let call = swap_router.exact_output(params);
        call
    }

    pub fn exact_output_single(&self, params: ExactOutputSingleParams) -> ContractCall<M, U256> {
        let swap_router = self.contract();
        let call = swap_router.exact_output_single(params);
        call
    }

    pub fn uniswap_v3_swap_callback(
        &self,
        amount_0_delta: I256,
        amount_1_delta: I256,
        data: Bytes,
    ) -> ContractCall<M, ()> {
        let swap_router = self.contract();
        let call = swap_router.uniswap_v3_swap_callback(amount_0_delta, amount_1_delta, data);
        call
    }
}
