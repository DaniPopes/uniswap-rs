use crate::{
    contracts::bindings::iweth::IWETH,
    errors::{Error, Result},
    utils::*,
    v2::Pair,
    Amount, Protocol, ProtocolType,
};
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, U256};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

#[cfg(feature = "addresses")]
use ethers_core::types::Chain;

/// Aggregates common methods to interact with the Uniswap v2 or v3 protocols and other utilities.
pub struct Dex<M> {
    /// The protocol.
    protocol: Protocol<M>,

    /// The address of the chain's wrapped native token.
    weth: Option<Address>,
}

impl<M> Clone for Dex<M> {
    fn clone(&self) -> Self {
        Self { protocol: self.protocol.clone(), weth: self.weth }
    }
}

impl<M> fmt::Debug for Dex<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dex").field("protocol", &self.protocol).field("weth", &self.weth).finish()
    }
}

impl<M: Middleware> Dex<M> {
    /// Creates a new instance of Dex using the provided addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        let protocol = Protocol::new(client, factory, router, protocol);
        Self { protocol, weth: None }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        Protocol::new_with_chain(client, chain, protocol).map(|protocol| {
            let weth = crate::contracts::addresses::try_address("WETH", chain);
            Self { protocol, weth }
        })
    }

    /// Returns a pointer to the client.
    pub fn client(&self) -> Arc<M> {
        self.protocol.client()
    }

    /// Returns the protocol.
    pub fn protocol(&self) -> &Protocol<M> {
        &self.protocol
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// The factory's address.
    pub fn factory_address(&self) -> Address {
        self.protocol.router_address()
    }

    /// Returns the contract call for creating a liquidity pair between two tokens.
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        self.protocol.create_pair(token_a, token_b)
    }

    /// Returns the pair for two token addresses.
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> Pair<M> {
        self.protocol.pair_for(token_a, token_b)
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// The router's address.
    pub fn router_address(&self) -> Address {
        self.protocol.router_address()
    }

    /// Returns the contract call for adding liquidity to a pair.
    pub fn add_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Option<Address>,
        deadline: Option<u64>,
    ) -> Result<ContractCall<M, (U256, U256, U256)>> {
        let sender = self.client().default_sender();
        let to = self.get_to(to);

        let deadline = get_deadline_opt(deadline);

        // TODO: Maths

        let mut call = self.protocol.add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )?;

        if let Some(from) = sender {
            call = call.from(from);
        }

        Ok(call)
    }

    /// Returns the contract call for removing liquidity from a pair.
    pub fn remove_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Option<Address>,
        deadline: Option<u64>,
    ) -> Result<ContractCall<M, (U256, U256)>> {
        let deadline = get_deadline_opt(deadline);

        let sender = self.client().default_sender();
        let to = self.get_to(to);

        // TODO: Maths

        let mut call = self.protocol.remove_liquidity(
            token_a,
            token_b,
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )?;

        if let Some(from) = sender {
            call = call.from(from);
        }

        Ok(call)
    }

    /// Returns the contract call for swapping two or more tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to swap, wrapped in a [helper Enum][Amount].
    ///
    /// * `slippage_tolerance` - The maximum price change, in percentage points, which may occur
    ///   while the transaction is pending, that you are willing to tolerate before it reverts.
    ///   `0.0` means no price change tolerated, while `100.0` means any price change is tolerated.
    ///
    /// * `path` - The path to take. `path.first()` or `path.last()` == [`NATIVE_ADDRESS`] indicates
    ///   intention to swap from or to the native token respectively.
    ///
    /// * `to` - The address that will receive the swap output. If `None`, it will default to the
    ///   default address of the client.
    ///
    /// * `deadline` - The number of seconds after which the transaction will revert. If `None`, it
    ///   will default to 1800 seconds.
    ///
    /// [`NATIVE_ADDRESS`]: crate::constants::NATIVE_ADDRESS
    pub async fn swap(
        &mut self,
        amount: Amount,
        slippage_tolerance: f32,
        path: &[Address],
        to: Option<Address>,
        deadline: Option<u64>,
    ) -> Result<ContractCall<M, Vec<U256>>> {
        if !(0.0..=100.0).contains(&slippage_tolerance) {
            return Err(Error::InvalidSlippage);
        }

        let sender = self.client().default_sender();
        let to = self.get_to(to);

        if path.len() < 2 {
            return Err(Error::InvalidPath);
        }

        // set weth
        let weth = match self.weth {
            Some(weth) => weth,
            None => {
                self.set_weth().await?;
                self.weth.expect("couldn't set weth")
            }
        };

        if path_eq(path, &weth) {
            return Err(Error::SwapToSelf);
        }

        let deadline = get_deadline_opt(deadline);

        let mut call =
            self.protocol.swap(amount, slippage_tolerance, path, to, deadline, weth).await?;

        if let Some(from) = sender {
            call = call.from(from);
        }

        Ok(call)
    }

    /* ------------------------------------------ WETH ------------------------------------------ */

    /// Returns the address of the wrapped native token.
    pub fn weth(&self) -> Option<Address> {
        self.weth
    }

    /// Sets the wrapped native token address by calling the WETH() method on the V2 router.
    ///
    /// Note that this does nothing on a V3 protocol.
    pub async fn set_weth(&mut self) -> Result<&mut Self> {
        match &self.protocol {
            Protocol::V2(v2) => {
                let weth = v2.router().contract().weth().call().await?;
                self.weth = Some(weth);
            }
            Protocol::V3 => {}
        };

        Ok(self)
    }

    /// Sets the wrapped native token address.
    pub fn set_weth_sync(&mut self, weth: Address) -> &mut Self {
        self.weth = Some(weth);

        self
    }

    /// Returns the contract call for `weth.deposit{ value: amount }()`.
    pub fn weth_deposit(&self, amount: U256) -> Result<ContractCall<M, ()>> {
        let address = match self.weth {
            Some(address) => address,
            None => return Err(Error::WethNotSet),
        };
        let client = self.client();
        let sender = client.default_sender();
        let weth = IWETH::new(address, client);
        let mut call = weth.deposit().value(amount);

        if let Some(sender) = sender {
            call = call.from(sender)
        }

        Ok(call)
    }

    /// Returns the contract call for `weth.withdraw(amount)`.
    pub fn weth_withdraw(&self, amount: U256) -> Result<ContractCall<M, ()>> {
        let address = match self.weth {
            Some(address) => address,
            None => return Err(Error::WethNotSet),
        };
        let client = self.client();
        let sender = client.default_sender();
        let weth = IWETH::new(address, client);
        let mut call = weth.withdraw(amount);

        if let Some(sender) = sender {
            call = call.from(sender)
        }

        Ok(call)
    }

    /// `to` > client.default_sender() > panic
    fn get_to(&self, to: Option<Address>) -> Address {
        let sender = self.client().default_sender();
        to.unwrap_or_else(|| {
            sender
                .expect("Must specify a `to` address if the client does not have a default sender")
        })
    }
}

/// first === last
fn path_eq(path: &[Address], weth: &Address) -> bool {
    let first = path.first().expect("path is empty");
    let last = path.last().expect("path is empty");

    if first == last {
        return true;
    }

    let fin = is_native(first);
    let lin = is_native(last);
    let fiw = first == weth;
    let liw = last == weth;

    (fin && liw) || (lin && fiw)
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::{constants::*, v2::Library as V2Library};
    use ethers_core::{
        abi::{self, ParamType, Token, Tokenize},
        types::{Bytes, I256},
    };
    use ethers_middleware::SignerMiddleware;
    use ethers_providers::{Http, Provider, MAINNET};
    use ethers_signers::LocalWallet;

    fn assert_approx_eq<A: Into<U256>, B: Into<U256>, C: Into<U256>>(a: A, b: B, c: C) {
        let a = I256::from_raw(a.into());
        let b = I256::from_raw(b.into());
        assert!((a - b).abs() < I256::from_raw(c.into()));
    }

    #[cfg(feature = "addresses")]
    fn default_dex() -> Dex<SignerMiddleware<Provider<Http>, LocalWallet>> {
        let provider: Provider<Http> = MAINNET.provider();
        let signer: LocalWallet =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse().unwrap();
        let client = SignerMiddleware::new(provider, signer);

        let chain = Chain::Mainnet;
        let dex_type = ProtocolType::UniswapV2;

        Dex::new_with_chain(Arc::new(client), chain, dex_type).unwrap()
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

    #[test]
    fn test_path_eq() {
        let weth = Address::repeat_byte(0xaa);

        // ne
        let path = vec![Address::random(), Address::random()];
        assert!(!path_eq(&path, &weth));

        let path = vec![weth, Address::random()];
        assert!(!path_eq(&path, &weth));

        let path = vec![Address::random(), weth];
        assert!(!path_eq(&path, &weth));

        // eq
        let path = vec![NATIVE_ADDRESS, weth];
        assert!(path_eq(&path, &weth));

        let path = vec![weth, NATIVE_ADDRESS];
        assert!(path_eq(&path, &weth));
    }

    #[tokio::test]
    #[ignore = "async test"]
    #[cfg(feature = "addresses")]
    async fn can_swap_infinite_slippage() {
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre = vec![Address::random(), Address::random()];
        let to_pre = Address::random();
        let deadline_pre = 1000;

        let contract_call =
            dex.swap(amount, 100.0, &path_pre, Some(to_pre), Some(deadline_pre)).await.unwrap();

        let calldata = contract_call.calldata().unwrap();

        println!("{calldata}");

        let args = decode_call(&calldata);
        println!("{args:?}");

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
    #[ignore = "async test"]
    #[cfg(feature = "addresses")]
    async fn can_swap_no_slippage() {
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let amount = Amount::ExactIn(amount_in_pre);
        let path_pre =
            vec![dex.weth.unwrap(), "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap()];

        let amounts_out = V2Library::get_amounts_out(
            dex.protocol().as_v2().unwrap().factory(),
            amount_in_pre,
            &path_pre,
        )
        .await
        .unwrap();

        let contract_call = dex.swap(amount, 0.0, &path_pre, None, None).await.unwrap();

        let calldata = contract_call.calldata().unwrap();

        let args = decode_call(&calldata);

        let amount_out_min = args[1].clone().into_uint().unwrap();

        // A block may get mined between the start of the test and now, skewing the reserves
        assert_approx_eq(amounts_out[1], amount_out_min, 1_000_000u64);
    }

    #[tokio::test]
    #[ignore = "async test"]
    #[cfg(feature = "addresses")]
    async fn can_swap_slippage() {
        let mut dex = default_dex();

        let amount_in_pre = U256::exp10(18);
        let path_pre =
            vec![dex.weth.unwrap(), "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap()];

        let amounts_out = V2Library::get_amounts_out(
            dex.protocol().as_v2().unwrap().factory(),
            amount_in_pre,
            &path_pre,
        )
        .await
        .unwrap();

        let amount = Amount::ExactIn(amount_in_pre);
        for i in 2..=10 {
            let slippage_tolerance = 100.0 / i as f32;

            let contract_call =
                dex.swap(amount, slippage_tolerance, &path_pre, None, None).await.unwrap();

            let calldata = contract_call.calldata().unwrap();

            let args = decode_call(&calldata);

            let amount_out_min = args[1].clone().into_uint().unwrap();
            let mult = 100.0 - slippage_tolerance;
            let mult_bps = U256::from((mult * 100.0) as u32);
            // A block may get mined between the start of the test and now, skewing the reserves
            assert_approx_eq(amount_out_min, (amounts_out[1] * mult_bps) / BPS_U256, 1_000_000u64);
        }
    }
}
