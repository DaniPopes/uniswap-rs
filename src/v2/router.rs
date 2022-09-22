use super::{Factory, Library};
use crate::{
    bindings::i_uniswap_v2_router_02::IUniswapV2Router02,
    constants::BPS_U256,
    errors::RouterResult,
    utils::{is_native_path, map_native},
    Amount, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Represents a UniswapV2 router.
#[derive(Clone, Copy, Debug)]
pub struct Router {
    /// The router address.
    address: Address,

    /// The router protocol.
    protocol: ProtocolType,
}

impl Router {
    /// Creates a new instance from using the provided address.
    pub fn new(address: Address, protocol: ProtocolType) -> Self {
        // assert!(protocol.is_v2(), "protocol must be v2");
        Self { address, protocol }
    }

    /// Creates a new instance using the provided chain.
    pub fn new_with_chain(chain: Chain, protocol: ProtocolType) -> Option<Self> {
        // assert!(protocol.is_v2(), "protocol must be v2");
        protocol.try_addresses(chain).0.map(|address| Self { address, protocol })
    }

    /// Returns the router contract.
    pub fn contract<M: Middleware>(&self, client: Arc<M>) -> IUniswapV2Router02<M> {
        IUniswapV2Router02::new(self.address, client)
    }

    /// Returns the router address.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the router protocol.
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }

    /// Generalized add_liquidity function for the various [UniswapV2Router] methods.
    /// Returns the contract call with the necessary parameters set (value, calldata).
    ///
    /// See documentation of [Dex] for more details on arguments.
    ///
    /// Note: this function does not perform many sanity checks and it should becalled by using the
    /// [Dex] struct.
    ///
    /// [UniswapV2Router]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/UniswapV2Router01.sol
    /// [Dex]: crate::Dex
    #[allow(clippy::too_many_arguments)]
    pub fn add_liquidity<M: Middleware>(
        &self,
        client: Arc<M>,
        token_a: Address,
        token_b: Address,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> RouterResult<ContractCall<M, (U256, U256, U256)>, M> {
        let router = self.contract(client);
        let (native_a, native_b) = is_native_path(&[token_a, token_b]);

        let call = if native_a ^ native_b {
            router.add_liquidity(
                token_a,
                token_b,
                amount_a_desired,
                amount_b_desired,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            )
        } else {
            let (token, amount_token_min, amount_token_desired, amount_eth_desired, amount_eth_min) =
                if native_a {
                    // token_a is ETH
                    (token_b, amount_b_min, amount_b_desired, amount_a_desired, amount_a_min)
                } else {
                    // token_b is ETH
                    (token_a, amount_a_min, amount_a_desired, amount_b_desired, amount_b_min)
                };
            router
                .add_liquidity_eth(
                    token,
                    amount_token_desired,
                    amount_token_min,
                    amount_eth_min,
                    to,
                    deadline,
                )
                .value(amount_eth_desired)
        };

        Ok(call)
    }

    /// Generalized remove_liquidity function for the various [UniswapV2Router] methods.
    /// Returns the contract call with the necessary parameters set (value, calldata).
    ///
    /// See documentation of [Dex] for more details on arguments.
    ///
    /// Note: this function does not perform many sanity checks and it should becalled by using the
    /// [Dex] struct.
    ///
    /// [UniswapV2Router]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/UniswapV2Router01.sol
    /// [Dex]: crate::Dex
    #[allow(clippy::too_many_arguments)]
    pub fn remove_liquidity<M: Middleware>(
        &self,
        client: Arc<M>,
        token_a: Address,
        token_b: Address,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> RouterResult<ContractCall<M, (U256, U256)>, M> {
        let router = self.contract(client);
        let (native_a, native_b) = is_native_path(&[token_a, token_b]);

        let call = if native_a ^ native_b {
            router.remove_liquidity(
                token_a,
                token_b,
                liquidity,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            )
        } else {
            let (token, amount_token_min, amount_eth_min) = if native_a {
                // token_a is ETH
                (token_b, amount_b_min, amount_a_min)
            } else {
                // token_b is ETH
                (token_a, amount_a_min, amount_b_min)
            };
            router.remove_liquidity_eth(
                token,
                liquidity,
                amount_token_min,
                amount_eth_min,
                to,
                deadline,
            )
        };

        Ok(call)
    }

    /// Generalized swap function for the various [UniswapV2Router] `swap[Exact]XFor[Exact]Y`.
    /// Returns the contract call with the necessary parameters set (value, calldata).
    ///
    /// See documentation of [Dex] for more details on arguments.
    ///
    /// Note: this function does not perform many sanity checks and it should becalled by using the
    /// [Dex] struct.
    ///
    /// [UniswapV2Router]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/UniswapV2Router01.sol
    /// [Dex]: crate::Dex
    #[allow(clippy::too_many_arguments)]
    pub async fn swap<M: Middleware>(
        &self,
        client: Arc<M>,
        factory: Factory,
        amount: Amount,
        slippage_tolerance: f32,
        mut path: Vec<Address>,
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> RouterResult<ContractCall<M, Vec<U256>>, M> {
        let router = self.contract(client.clone());
        let (from_native, to_native) = is_native_path(&path);
        map_native(&mut path, weth);
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
