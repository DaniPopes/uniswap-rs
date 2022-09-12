use crate::{
    bindings::i_uniswap_v2_router_02::IUniswapV2Router02, errors::RouterError,
    utils::is_native_path, Amount,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

type Result<T, M> = std::result::Result<T, RouterError<M>>;

/// TODO
#[derive(Debug)]
pub struct Router<M> {
    /// The client.
    client: Arc<M>,

    /// The router address.
    address: Address,
}

impl<M> Clone for Router<M> {
    fn clone(&self) -> Self {
        Self { client: self.client.clone(), address: self.address }
    }
}

impl<M: Middleware> Router<M> {
    /// TODO
    pub fn new(client: Arc<M>, address: Address) -> Self {
        Self { client, address }
    }

    /// TODO
    pub async fn add_liquidity(&mut self) -> Result<ContractCall<M, Vec<U256>>, M> {
        todo!()
    }

    /// TODO
    pub async fn swap(
        &mut self,
        amount: Amount,
        slippage_tolerance: f32,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, Vec<U256>>, M> {
        let router = IUniswapV2Router02::new(self.address, self.client.clone());
        let (from_native, to_native) = is_native_path(&path);
        // let call = match amount {
        //     Amount::ExactIn(amount_in) => {
        //         let amount_out_min = if slippage_tolerance == 100.0 {
        //             U256::zero()
        //         } else {
        //             // TODO: Optimize external calls
        //             let last_amount_out = *Library::get_amounts_out(
        //                 self.factory,
        //                 amount_in,
        //                 path.clone(),
        //                 self.client.clone(),
        //             )
        //             .await?
        //             .last()
        //             .expect("path is empty");
        //             if slippage_tolerance == 0.0 {
        //                 last_amount_out
        //             } else {
        //                 let mult = 100.0 - slippage_tolerance;
        //                 let mult_bps = U256::from((mult * 100.0) as u32);
        //                 (last_amount_out * mult_bps) / BPS_U256
        //             }
        //         };

        //         if from_native {
        //             router
        //                 .swap_exact_eth_for_tokens(amount_out_min, path, to, deadline)
        //                 .value(amount_in)
        //         } else if to_native {
        //             router.swap_exact_tokens_for_eth(amount_in, amount_out_min, path, to,
        // deadline)         } else {
        //             router.swap_exact_tokens_for_tokens(
        //                 amount_in,
        //                 amount_out_min,
        //                 path,
        //                 to,
        //                 deadline,
        //             )
        //         }
        //     }
        //     Amount::ExactOut(amount_out) => {
        //         let amount_in_max = if slippage_tolerance == 100.0 {
        //             U256::max_value()
        //         } else {
        //             // TODO: Optimize external calls
        //             let first_amount_in = *Library::get_amounts_in(
        //                 self.factory,
        //                 amount_out,
        //                 path.clone(),
        //                 self.client.clone(),
        //             )
        //             .await?
        //             .first()
        //             .expect("path is empty");
        //             if slippage_tolerance == 0.0 {
        //                 first_amount_in
        //             } else {
        //                 let mult = 1.0 / (100.0 - slippage_tolerance);
        //                 let mult_bps = U256::from((mult * 100.0).round() as u32);
        //                 (first_amount_in * mult_bps) / BPS_U256
        //             }
        //         };

        //         if from_native {
        //             router
        //                 .swap_eth_for_exact_tokens(amount_out, path, to, deadline)
        //                 .value(amount_out)
        //         } else if to_native {
        //             router.swap_tokens_for_exact_eth(amount_out, amount_in_max, path, to,
        // deadline)         } else {
        //             router.swap_tokens_for_exact_tokens(
        //                 amount_out,
        //                 amount_in_max,
        //                 path,
        //                 to,
        //                 deadline,
        //             )
        //         }
        //     }
        // };

        // Ok(call)

        todo!()
    }
}
