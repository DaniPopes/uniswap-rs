use super::{Factory, Library};
use crate::{
    bindings::i_uniswap_v2_router_02::IUniswapV2Router02, constants::BPS_U256, errors::RouterError,
    utils::is_native_path, Amount, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

type Result<T, M> = std::result::Result<T, RouterError<M>>;

/// TODO
#[derive(Clone, Copy, Debug)]
pub struct Router {
    /// The router address.
    address: Address,

    /// The router protocol.
    _protocol: ProtocolType,
}

impl Router {
    /// Creates a new instance of Router from an address.
    pub fn new(address: Address, protocol: ProtocolType) -> Self {
        // assert!(protocol.is_v2(), "protocol must be v2");
        Self { address, _protocol: protocol }
    }

    /// Creates a new instance of Router from an address.
    pub fn new_with_chain(chain: Chain, protocol: ProtocolType) -> Option<Self> {
        // assert!(protocol.is_v2(), "protocol must be v2");
        protocol.try_addresses(chain).0.map(|address| Self { address, _protocol: protocol })
    }

    /// Returns the router contract.
    pub fn contract<M: Middleware>(&self, client: Arc<M>) -> IUniswapV2Router02<M> {
        IUniswapV2Router02::new(self.address, client)
    }

    /// TODO
    pub async fn add_liquidity<M: Middleware>(
        &self,
        _client: Arc<M>,
    ) -> Result<ContractCall<M, Vec<U256>>, M> {
        todo!("add_liquidity is not yet implemented")
    }

    /// TODO
    #[allow(clippy::too_many_arguments)]
    pub async fn swap<M: Middleware>(
        &self,
        client: Arc<M>,
        factory: Factory,
        amount: Amount,
        slippage_tolerance: f32,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, Vec<U256>>, M> {
        let router = IUniswapV2Router02::new(self.address, client.clone());
        let (from_native, to_native) = is_native_path(&path);
        let call = match amount {
            Amount::ExactIn(amount_in) => {
                let amount_out_min = if slippage_tolerance == 100.0 {
                    U256::zero()
                } else {
                    let last_amount_out =
                        *Library::get_amounts_out(client, factory, amount_in, path.clone())
                            .await?
                            .last()
                            .expect("path is empty");
                    if slippage_tolerance == 0.0 {
                        last_amount_out
                    } else {
                        let mult = 100.0 - slippage_tolerance;
                        let mult_bps = U256::from((mult * 100.0) as u32);
                        (last_amount_out * mult_bps) / BPS_U256
                    }
                };

                if from_native {
                    router
                        .swap_exact_eth_for_tokens(amount_out_min, path, to, deadline)
                        .value(amount_in)
                } else if to_native {
                    router.swap_exact_tokens_for_eth(amount_in, amount_out_min, path, to, deadline)
                } else {
                    router.swap_exact_tokens_for_tokens(
                        amount_in,
                        amount_out_min,
                        path,
                        to,
                        deadline,
                    )
                }
            }
            Amount::ExactOut(amount_out) => {
                let amount_in_max = if slippage_tolerance == 100.0 {
                    U256::max_value()
                } else {
                    let first_amount_in =
                        *Library::get_amounts_in(client, factory, amount_out, path.clone())
                            .await?
                            .first()
                            .expect("path is empty");
                    if slippage_tolerance == 0.0 {
                        first_amount_in
                    } else {
                        let mult = 1.0 / (100.0 - slippage_tolerance);
                        let mult_bps = U256::from((mult * 100.0).round() as u32);
                        (first_amount_in * mult_bps) / BPS_U256
                    }
                };

                if from_native {
                    router
                        .swap_eth_for_exact_tokens(amount_out, path, to, deadline)
                        .value(amount_out)
                } else if to_native {
                    router.swap_tokens_for_exact_eth(amount_out, amount_in_max, path, to, deadline)
                } else {
                    router.swap_tokens_for_exact_tokens(
                        amount_out,
                        amount_in_max,
                        path,
                        to,
                        deadline,
                    )
                }
            }
        };

        Ok(call)
    }
}
