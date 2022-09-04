use crate::contracts::address;
use crate::{bindings::i_uniswap_v2_router_02::IUniswapV2Router02, factory::Factory};
use crate::{UniswapV2Library, UniswapV2LibraryError};
use ethers::prelude::{builders::ContractCall, *};
use std::{sync::Arc, time::SystemTime};

mod protocol;
pub use protocol::Protocol;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Amount {
    /// Swap X Token1 for any Token2
    ExactIn(U256),
    /// Swap any Token1 for X Token2
    ExactOut(U256),
}

#[derive(Debug, thiserror::Error)]
pub enum DexError<M: Middleware> {
    #[error(transparent)]
    UniswapV2LibraryError(#[from] UniswapV2LibraryError),

    #[error(transparent)]
    ContractError(#[from] ContractError<M>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error("Slippage must be in range: 0.0..=100.0")]
    InvalidSlippage,

    #[error("Cannot swap a token into itself")]
    SwapToSelf,
}

pub type Result<T, M> = std::result::Result<T, DexError<M>>;

const MIN_DEADLINE_SECONDS: u64 = 30;
const DEFAULT_DEADLINE_SECONDS: u64 = 1800;
const BPS_U256: U256 = U256([10_000u64, 0, 0, 0]);

#[derive(Clone, Debug)]
pub struct Dex<M> {
    /// The node and signer.
    client: Arc<M>,

    /// The dex swap router.
    router: Address,

    /// The dex liquidity pair factory.
    factory: Factory,

    /// The chain's wrapped native token.
    weth: Address,

    /// The protocol.
    #[allow(dead_code)]
    protocol: Protocol,
}

// TODO: UniV3. Separate as UniV2 and UniV3 Dex structs?
// TODO: Common Dex trait?
// TODO: Support for more chains
impl<M: Middleware> Dex<M> {
    pub fn new(client: Arc<M>, chain: Chain, protocol: Protocol) -> Self {
        if client.default_sender().is_none() {
            panic!("Client is not a signer");
        }
        let (router_address, factory_address) = protocol.get_addresses(chain);
        // TODO: Query factory for .WETH()
        let weth_address = address("WETH", chain);
        Self {
            client,
            router: router_address,
            factory: Factory::new(Some(factory_address), Some(chain), protocol),
            weth: weth_address,
            protocol,
        }
    }

    pub fn new_with_factory(_factory: Factory) -> Self {
        todo!()
    }

    /// Generalized swap function for the various [UniswapV2Router] `swap[Exact]XFor[Exact]Y`.
    /// Returns the contract call with the necessary parameters set (value, calldata).
    ///
    /// `slippage_tolerance` is the maximum price change, in percentage points, which may occur
    /// while the transaction is pending, that you are willing to tolerate before the
    /// transaction reverts. `0.0` means no price change tolerated, while `100.0` means any price
    /// change is tolerated.
    ///
    /// Using a `path[0]` or `path[path.length - 1]` == `Address::zero()` indicates intention to
    /// swap from or to the native coin respectively.
    ///
    /// `to` is the address that will receive the swap output. If [`None`], it will default to
    /// [`self.client.default_address()`].
    ///
    /// The transaction will revert if it is pending for more than `deadline` seconds. If [`None`],
    /// it will default to [`DEFAULT_DEADLINE_SECONDS`].
    ///
    /// [UniswapV2Router]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/UniswapV2Router01.sol
    /// [`self.client.default_address()`]: Middleware
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: Vec<Address>,
        to: Option<Address>,
        deadline: Option<u64>,
    ) -> Result<ContractCall<M, Vec<U256>>, M> {
        if !(0.0..=100.0).contains(&slippage_tolerance) {
            return Err(DexError::InvalidSlippage);
        }

        let to = to.unwrap_or_else(|| {
            self.client
                .default_sender()
                .expect("Missing self.client.default_sender()")
        });

        if path.len() < 2 {
            return Err(DexError::UniswapV2LibraryError(
                UniswapV2LibraryError::InvalidPath,
            ));
        }

        // can unwrap since we just asserted path.len() >= 2
        let first_address = *path.first().expect("Path is empty, should not happen");
        let last_address = *path.last().expect("Path is empty, should not happen");

        let from_native = first_address.is_zero();
        let to_native = last_address.is_zero();

        if first_address == last_address {
            return Err(DexError::UniswapV2LibraryError(
                UniswapV2LibraryError::InvalidPath,
            ));
        }

        // map Address::zero() to self.weth
        let path: Vec<H160> = path
            .iter()
            .map(|addr| if addr.is_zero() { self.weth } else { *addr })
            .collect();

        // let total_path = [first_address, last_address];

        let deadline = {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let deadline = now + deadline.unwrap_or(DEFAULT_DEADLINE_SECONDS);
            let min_deadline = now + MIN_DEADLINE_SECONDS;
            if deadline < min_deadline {
                min_deadline
            } else {
                deadline
            }
        }
        .into();

        let router = IUniswapV2Router02::new(self.router, self.client.clone());
        let contract_call = match amount {
            Amount::ExactIn(amount_in) => {
                let amount_out_min = if slippage_tolerance == 100.0 {
                    U256::zero()
                } else {
                    // TODO: Optimize external calls
                    let last_amount_out = *UniswapV2Library::get_amounts_out(
                        self.factory,
                        amount_in,
                        path.clone(),
                        self.client.clone(),
                    )
                    .await?
                    .last()
                    .expect("Path is empty, should not happen");
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
                    // TODO: Optimize external calls
                    let first_amount_in = *UniswapV2Library::get_amounts_in(
                        self.factory,
                        amount_out,
                        path.clone(),
                        self.client.clone(),
                    )
                    .await?
                    .first()
                    .expect("Path is empty, should not happen");
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

        Ok(contract_call)
    }
}

#[cfg(test)]
mod tests {
    use ethers::abi::{ParamType, Token, Tokenize};

    use super::*;

    fn default_dex() -> Dex<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
        let provider: Provider<Http> = MAINNET.provider();
        let signer: LocalWallet =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .parse()
                .unwrap();
        let client = SignerMiddleware::new(provider, signer);

        let chain = Chain::Mainnet;
        let dex_type = Protocol::UniswapV2;

        Dex::new(Arc::new(client), chain, dex_type)
    }

    fn decode_call(calldata: &Bytes) -> Vec<Token> {
        abi::decode(
            &[
                ParamType::Uint(256),
                ParamType::Uint(256),
                ParamType::Array(Box::new(ParamType::Address)),
                ParamType::Address,
                ParamType::Uint(256),
            ],
            &calldata[4..],
        )
        .unwrap()
    }

    fn decode_call_with_value(calldata: &Bytes) -> Vec<Token> {
        abi::decode(
            &[
                ParamType::Uint(256),
                ParamType::Array(Box::new(ParamType::Address)),
                ParamType::Address,
                ParamType::Uint(256),
            ],
            &calldata[4..],
        )
        .unwrap()
    }

    #[tokio::test]
    async fn can_swap_infinite_slippage() {
        let dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre = [Address::random(), Address::random()].to_vec();
        let to_pre = Address::random();
        let deadline_pre = 1000;

        let contract_call = dex
            .swap(
                amount,
                100.0,
                path_pre.clone(),
                Some(to_pre),
                Some(deadline_pre),
            )
            .await
            .unwrap();

        let calldata = contract_call.calldata().unwrap();

        println!("{}", calldata);

        let args = decode_call(&calldata);
        println!("{:?}", args);

        let amount_in = args[0].clone().into_uint().unwrap();
        let amount_out_min = args[1].clone().into_uint().unwrap();
        let path = args[2].clone().into_array().unwrap();
        let to = args[3].clone().into_address().unwrap();
        // let deadline = args[4].clone().into_uint().unwrap();

        let path_pre = {
            if let Token::Array(a) = path_pre.into_tokens()[0].clone() {
                Some(a)
            } else {
                None
            }
        };

        assert_eq!(calldata[0..4], [0x38, 0xed, 0x17, 0x39]);
        assert_eq!(amount_in, amount_in_pre);
        assert!(amount_out_min.is_zero());
        assert_eq!(path, path_pre.unwrap());
        assert_eq!(to, to_pre);
    }

    #[tokio::test]
    async fn can_swap_no_slippage() {
        let dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre = [dex.weth, address("USDC", Chain::Mainnet)].to_vec();

        let amounts_out = UniswapV2Library::get_amounts_out(
            dex.factory,
            amount_in_pre,
            path_pre.clone(),
            dex.client.clone(),
        )
        .await
        .unwrap();

        let contract_call = dex
            .swap(amount, 0.0, path_pre.clone(), None, None)
            .await
            .unwrap();

        let calldata = contract_call.calldata().unwrap();

        let args = decode_call(&calldata);

        let amount_out_min = args[1].clone().into_uint().unwrap();

        assert_eq!(amounts_out[1].clone(), amount_out_min);
    }

    #[tokio::test]
    async fn can_swap_slippage() {
        let dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let path_pre = [dex.weth, address("USDC", Chain::Mainnet)].to_vec();

        let amounts_out = UniswapV2Library::get_amounts_out(
            dex.factory,
            amount_in_pre,
            path_pre.clone(),
            dex.client.clone(),
        )
        .await
        .unwrap();

        let amount = Amount::ExactIn(amount_in_pre);
        for i in 2..=10 {
            let slippage_tolerance = 100.0 / i as f32;

            let contract_call = dex
                .swap(amount, slippage_tolerance, path_pre.clone(), None, None)
                .await
                .unwrap();

            let calldata = contract_call.calldata().unwrap();

            let args = decode_call(&calldata);

            let amount_out_min = args[1].clone().into_uint().unwrap();
            let mult = 100.0 - slippage_tolerance;
            let mult_bps = U256::from((mult * 100.0) as u32);
            assert_eq!(amount_out_min, (amounts_out[1] * mult_bps) / BPS_U256);
        }
    }
}
