use crate::{
    bindings::iweth::IWETH,
    constants::*,
    contracts::try_address,
    errors::{DexError, LibraryError},
    utils::*,
    v2::{Factory, Router, V2Protocol},
    Amount, Protocol, ProtocolType,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

/// Type alias for Result<T, DexError<M>>.
type Result<T, M> = std::result::Result<T, DexError<M>>;

/// Aggregates common methods to interact with the Uniswap v2 or v3 protocols and other utilities.
#[derive(Clone, Debug)]
pub struct Dex<M> {
    /// The node and signer.
    client: Arc<M>,

    /// The protocol.
    protocol: Protocol<M>,

    /// The address of the chain's wrapped native token.
    weth: Option<Address>,
}

// TODO: UniV3
impl<M: Middleware> Dex<M> {
    /// Creates a new instance of Dex from a chain.
    ///
    /// # Panics
    ///
    /// When the protocol's addresses could not be found.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let protocol = match protocol {
            p if p.is_v2() => Protocol::V2(V2Protocol::new(client.clone(), factory, router, p)),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol {:?} is neither v2 nor v3", p),
        };
        Self { client, weth: None, protocol }
    }

    /// Creates a new instance of Dex from a chain.
    ///
    /// # Panics
    ///
    /// When the protocol's addresses could not be found.
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Self {
        let protocol = match protocol {
            p if p.is_v2() => Protocol::V2(V2Protocol::new_with_chain(client.clone(), chain, p)),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol {:?} is neither v2 nor v3", p),
        };
        let weth = try_address("WETH", chain);
        Self { client, weth, protocol }
    }

    /// Returns the client.
    pub fn client(&self) -> Arc<M> {
        self.client.clone()
    }

    /// Returns the protocol.
    pub fn protocol(&self) -> Protocol<M> {
        self.protocol.clone()
    }

    /// Returns the factory.
    pub fn factory(&self) -> Factory {
        self.protocol.factory()
    }

    /// Returns the router.
    pub fn router(&self) -> Router {
        self.protocol.router()
    }

    /// Returns the address of the wrapped native token.
    pub fn weth(&self) -> Option<Address> {
        self.weth
    }

    /// Sets the wrapped native token address by calling the WETH() method on the router.
    pub async fn set_weth(&mut self) -> Result<&mut Self, M> {
        self.weth = Some(self.protocol.router().contract(self.client.clone()).weth().call().await?);

        Ok(self)
    }

    /// Sets the wrapped native token address.
    pub fn set_weth_sync(&mut self, weth: Address) -> &mut Self {
        self.weth = Some(weth);

        self
    }

    /// Swaps an amount of tokens for another token, within the provided deadline.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to swap, wrapped in a [helper Enum][Amount].
    ///
    /// * `slippage_tolerance` - The maximum price change, in percentage points, which may occur
    ///   while the transaction is pending, that you are willing to tolerate before it reverts.
    ///   `0.0` means no price change tolerated, while `100.0` means any price change is tolerated.
    ///
    /// * `path` - The path to take. `path.first()` or `path.last()` == [`NATIVE_TOKEN_ADDRESS`]
    ///   indicates intention to swap from or to the native token respectively.
    ///
    /// * `to` - The address that will receive the swap output. If `None`, it will default to the
    ///   default address of the client.
    ///
    /// * `deadline` - The number of seconds after which the transaction will revert. If `None`, it
    ///   will default to 1800 seconds.
    ///
    /// [`NATIVE_TOKEN_ADDRESS`]: crate::constants::NATIVE_TOKEN_ADDRESS
    pub async fn swap(
        &mut self,
        amount: Amount,
        slippage_tolerance: f32,
        mut path: Vec<Address>,
        to: Option<Address>,
        deadline: Option<u64>,
    ) -> Result<ContractCall<M, Vec<U256>>, M> {
        if !(0.0..=100.0).contains(&slippage_tolerance) {
            return Err(DexError::InvalidSlippage)
        }

        let sender = self.client.default_sender();
        let to = to.unwrap_or_else(|| {
            sender
                .expect("Must specify a `to` address if the client does not have a default sender")
        });

        if path.len() < 2 {
            return Err(LibraryError::InvalidPath.into())
        }

        // set weth
        let weth = match self.weth {
            Some(weth) => weth,
            None => {
                self.set_weth().await?;
                self.weth.expect("couldn't set weth")
            }
        };

        // map eth to weth
        for address in path.iter_mut() {
            if is_native(address) {
                *address = weth
            }
        }

        // after map so it counts both eth and weth as the same thing
        if path.first().expect("path is empty") == path.last().expect("path is empty") {
            return Err(DexError::SwapToSelf)
        }

        let deadline: U256 = {
            let now = now().as_secs();

            now + deadline.unwrap_or(DEFAULT_DEADLINE_SECONDS)
        }
        .into();

        match self.protocol {
            Protocol::V2(ref protocol) => {
                let mut call =
                    protocol.swap(amount, slippage_tolerance, path, to, deadline).await?;

                if let Some(from) = sender {
                    call = call.from(from);
                }

                Ok(call)
            }
            Protocol::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// Returns a `weth.deposit()` [ContractCall].
    pub fn weth_deposit(&self, amount: U256) -> Result<ContractCall<M, ()>, M> {
        let address = match self.weth {
            Some(address) => address,
            None => return Err(DexError::WethNotSet),
        };
        let weth = IWETH::new(address, self.client.clone());
        let mut call = weth.deposit().value(amount);

        if let Some(sender) = self.client.default_sender() {
            call = call.from(sender)
        }

        Ok(call)
    }

    /// Returns a `weth.withdraw(uint256)` [ContractCall].
    pub fn weth_withdraw(&self, amount: U256) -> Result<ContractCall<M, ()>, M> {
        let address = match self.weth {
            Some(address) => address,
            None => return Err(DexError::WethNotSet),
        };
        let weth = IWETH::new(address, self.client.clone());
        let mut call = weth.withdraw(amount);

        if let Some(sender) = self.client.default_sender() {
            call = call.from(sender)
        }

        Ok(call)
    }
}

#[cfg(test)]
mod tests {
    use crate::{contracts::address, v2::Library as V2Library};
    use ethers::abi::{ParamType, Token, Tokenize};

    use super::*;

    fn default_dex() -> Dex<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
        let provider: Provider<Http> = MAINNET.provider();
        let signer: LocalWallet =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse().unwrap();
        let client = SignerMiddleware::new(provider, signer);

        let chain = Chain::Mainnet;
        let dex_type = ProtocolType::UniswapV2;

        Dex::new_with_chain(Arc::new(client), chain, dex_type)
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

    #[allow(unused)]
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
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre = vec![Address::random(), Address::random()];
        let to_pre = Address::random();
        let deadline_pre = 1000;

        let contract_call = dex
            .swap(amount, 100.0, path_pre.clone(), Some(to_pre), Some(deadline_pre))
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
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre = vec![dex.weth.unwrap(), address("USDC", Chain::Mainnet)];

        let amounts_out: Vec<U256> = V2Library::get_amounts_out(
            dex.client(),
            dex.factory(),
            amount_in_pre,
            path_pre.clone(),
        )
        .await
        .unwrap();

        let contract_call = dex.swap(amount, 0.0, path_pre.clone(), None, None).await.unwrap();

        let calldata = contract_call.calldata().unwrap();

        let args = decode_call(&calldata);

        let amount_out_min = args[1].clone().into_uint().unwrap();

        // A block may get mined between the start of the test and now, skewing the reserves
        let a = I256::from_raw(amounts_out[1].clone());
        let b = I256::from_raw(amount_out_min);
        assert!((a - b).abs() < I256::from(1_000_000u64));
    }

    #[tokio::test]
    async fn can_swap_slippage() {
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let path_pre = vec![dex.weth.unwrap(), address("USDC", Chain::Mainnet)];

        let amounts_out = V2Library::get_amounts_out(
            dex.client(),
            dex.factory(),
            amount_in_pre,
            path_pre.clone(),
        )
        .await
        .unwrap();

        let amount = Amount::ExactIn(amount_in_pre);
        for i in 2..=10 {
            let slippage_tolerance = 100.0 / i as f32;

            let contract_call =
                dex.swap(amount, slippage_tolerance, path_pre.clone(), None, None).await.unwrap();

            let calldata = contract_call.calldata().unwrap();

            let args = decode_call(&calldata);

            let amount_out_min = args[1].clone().into_uint().unwrap();
            let mult = 100.0 - slippage_tolerance;
            let mult_bps = U256::from((mult * 100.0) as u32);
            assert_eq!(amount_out_min, (amounts_out[1] * mult_bps) / BPS_U256);
        }
    }
}
