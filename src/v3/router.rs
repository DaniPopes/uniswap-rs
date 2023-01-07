#![allow(unreachable_pub, unused)]

use crate::contracts::bindings::i_swap_router::*;
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, Bytes, Chain, H256, I256, U256};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

contract_struct! {
    /// Represents a UniswapV3 SwapRouter.
    pub struct Router<M> {
        /// The router contract.
        contract: ISwapRouter<M>,
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
