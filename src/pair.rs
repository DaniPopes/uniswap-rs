use crate::{
    bindings::i_uniswap_v2_pair::IUniswapV2Pair, factory::Factory, UniswapV2Library,
    UniswapV2LibraryError,
};
use ethers::{
    abi::Token,
    core::abi::Detokenize,
    prelude::{builders::ContractCall, *},
};
use std::sync::Arc;

type Tokens = (Address, Address);
type Reserves = (u128, u128, u32);

#[derive(Debug, thiserror::Error)]
pub enum PairError<M: Middleware> {
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    UniswapV2LibraryError(#[from] UniswapV2LibraryError),
    #[error(transparent)]
    MulticallError(#[from] MulticallError<M>),
}

type Result<T, M> = std::result::Result<T, PairError<M>>;

/// Represents a UniswapV2 liquidity pair, composed of 2 different ERC20 tokens.
///
/// # Example
///
/// ```no_run
/// use ethers::prelude::*;
/// use uniswap::Pair;
/// use std::{convert::TryFrom, sync::Arc};
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// // Example UniswapV2 liquidity pair on mainnet
/// let weth_usdc_address: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse().unwrap();
/// let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
/// let client = Arc::new(client);
/// let mut pair = Pair::new(client, weth_usdc_address);
///
/// pair.sync(true, true).await?;
///
/// let reserves = pair.reserves().unwrap();
/// assert!(reserves.0 > 0);
/// assert!(reserves.1 > 0);
/// assert!(reserves.2 > 0);
///
/// assert!(pair.deployed());
/// # Ok(())
/// # }
#[derive(Clone)]
pub struct Pair<M> {
    /// The client.
    client: Arc<M>,

    /// The pair address. Might not be currently deployed.
    address: Address,

    /// The ordered tokens of the pair.
    tokens: Option<Tokens>,

    /// Whether the pair is currently deployed in the client's network.
    deployed: bool,

    /// The token reserves of the pair.
    reserves: Option<Reserves>,
}

// Skip client in formatting
impl<M> std::fmt::Debug for Pair<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pair")
            .field("address", &self.address)
            .field("tokens", &self.tokens)
            .field("deployed", &self.deployed)
            .field("reserves", &self.reserves)
            .finish()
    }
}

impl<M: Middleware> From<Pair<M>> for IUniswapV2Pair<M> {
    fn from(pair: Pair<M>) -> Self {
        Self::new(pair.address, pair.client)
    }
}

impl<M: Middleware> From<&Pair<M>> for IUniswapV2Pair<M> {
    fn from(pair: &Pair<M>) -> Self {
        Self::new(pair.address, pair.client.clone())
    }
}

impl<M: Middleware> From<&mut Pair<M>> for IUniswapV2Pair<M> {
    fn from(pair: &mut Pair<M>) -> Self {
        Self::new(pair.address, pair.client.clone())
    }
}

impl<M: Middleware> Pair<M> {
    pub fn new(client: Arc<M>, address: Address) -> Self {
        Self { client, address, tokens: None, deployed: false, reserves: None }
    }

    pub fn new_with_tokens(
        client: Arc<M>,
        factory: Factory,
        token0: Address,
        token1: Address,
    ) -> Result<Self, M> {
        let (token0, token1) = UniswapV2Library::sort_tokens(token0, token1)?;
        let address = UniswapV2Library::pair_for(factory, token0, token1)?;

        Ok(Self {
            client,
            address,
            tokens: Some((token0, token1)),
            deployed: false,
            reserves: None,
        })
    }

    /// Returns the address of the pair.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns whether the pair has been deployed. If it hasn't been synced yet, this will be equal
    /// to false.
    pub fn deployed(&self) -> bool {
        self.deployed
    }

    pub fn tokens(&self) -> Option<Tokens> {
        self.tokens
    }

    /// Returns the reserves of the pair. If it hasn't been synced yet, this will be equal to (0, 0,
    /// 0).
    pub fn reserves(&self) -> Option<Reserves> {
        self.reserves
    }

    /// Returns the contract calls for getting the addresses of the pair's tokens.
    pub fn get_tokens(&self) -> (ContractCall<M, Address>, ContractCall<M, Address>) {
        let pair = IUniswapV2Pair::from(self);
        (pair.token_0(), pair.token_1())
    }

    /// Returns the contract call for getting the reserves of the pair.
    pub fn get_reserves(&self) -> ContractCall<M, Reserves> {
        IUniswapV2Pair::from(self).get_reserves()
    }

    /// Syncs the tokens and reserves of the pair by querying the blockchain.
    ///
    /// Assumes that any call failure means the pair has not been deployed yet.
    pub async fn sync(&mut self, sync_tokens: bool, sync_reserves: bool) -> Result<&mut Self, M> {
        // let sync_tokens = sync_tokens || self.tokens.is_none() || !self.deployed;
        // let sync_reserves = sync_reserves || self.reserves.is_none();

        let multicall = Multicall::new(self.client.clone(), None).await?;
        let mut multicall = multicall.version(MulticallVersion::Multicall3);

        if sync_tokens {
            let calls = self.get_tokens();
            multicall.add_call(calls.0, true);
            multicall.add_call(calls.1, true);
        }

        if sync_reserves {
            multicall.add_call(self.get_reserves(), true);
        }

        let result = multicall.call_raw().await?;

        // Assume any call failure means the contract has not been deployed yet
        match (sync_tokens, sync_reserves) {
            (true, true) => {
                let tokens = parse_tokens_result(result[0..2].to_vec())?;
                let reserves = parse_reserves_result(result[2..].to_vec())?;

                if tokens.is_none() || reserves.is_none() {
                    self.tokens = None;
                    self.deployed = false;
                    return Ok(self)
                }

                self.deployed = true;
                self.tokens = tokens;
                self.reserves = reserves;
            }
            (true, false) => {
                let tokens = parse_tokens_result(result)?;

                if tokens.is_none() {
                    self.tokens = None;
                    self.deployed = false;
                    return Ok(self)
                }

                self.deployed = true;
                self.tokens = tokens;
            }
            (false, true) => {
                let reserves = parse_reserves_result(result)?;

                if reserves.is_none() {
                    self.tokens = None;
                    self.deployed = false;
                    return Ok(self)
                }

                self.deployed = true;
                self.reserves = reserves;
            }
            (false, false) => {}
        }

        Ok(self)
    }
}

/// Parses (bool, String) from a vector of tokens.
fn parse_errors(tokens: Vec<Token>) -> Vec<Option<String>> {
    type ErrorResult = (bool, String);
    let mut msgs = vec![];

    for token in tokens {
        match ErrorResult::from_tokens(vec![token]) {
            Ok((_, msg)) => msgs.push(Some(msg)),
            Err(_) => msgs.push(None),
        }
    }

    msgs
}

/// Parses a multicall result from a vector of tokens, returning None if the call returned an
/// error.
fn parse_result<M: Middleware, D: Detokenize>(tokens: Vec<Token>) -> Result<Option<D>, M> {
    let res = D::from_tokens(tokens.clone());
    match res {
        Err(e) => {
            // Failed to decode
            let errors = parse_errors(tokens);
            if errors.iter().any(|s| s.is_none()) {
                // Failed to decode errors too
                Err(ContractError::DetokenizationError(e).into())
            } else {
                // All calls failed while allowed
                Ok(None)
            }
        }
        Ok(res) => Ok(Some(res)),
    }
}

/// Parses a multicall result of Pair::get_tokens(), returning None if the call returned an
/// error.
fn parse_tokens_result<M: Middleware>(tokens: Vec<Token>) -> Result<Option<Tokens>, M> {
    type TokensResult = ((bool, Address), (bool, Address));
    let res: Option<TokensResult> = parse_result(tokens)?;

    match res {
        Some(res) => {
            if res.0 .0 && res.1 .0 {
                Ok(Some((res.0 .1, res.1 .1)))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

/// Parses a multicall result of Pair::get_reserves(), returning None if the call returned an
/// error.
fn parse_reserves_result<M: Middleware>(tokens: Vec<Token>) -> Result<Option<Reserves>, M> {
    type ReservesResult = (bool, Reserves);
    let res: Option<ReservesResult> = parse_result(tokens)?;

    match res {
        Some(res) => {
            if res.0 {
                Ok(Some(res.1))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{contracts::address, Protocol};

    fn default_pair() -> Pair<Provider<Http>> {
        let chain = Chain::Mainnet;
        let weth = address("WETH", chain);
        let usdc = address("USDC", chain);
        let provider = MAINNET.provider();
        let client = Arc::new(provider);
        let factory = Factory::new(None, Some(chain), Protocol::UniswapV2);
        let pair = Pair::new_with_tokens(client, factory, weth, usdc).unwrap();
        pair
    }

    #[test]
    #[allow(unused)]
    fn test_parsing() {
        let addresses = (Address::random(), Address::random());
        let tokens = vec![Token::Address(addresses.0), Token::Address(addresses.1)];
        let reserve_uints = (69u128, 420u128, 1337u32);
        let reserves = vec![
            Token::Uint(reserve_uints.0.into()),
            Token::Uint(reserve_uints.1.into()),
            Token::Uint(reserve_uints.2.into()),
        ];
        let error_message = "Error message".to_string();

        type SuccessResult = ((bool, Address), (bool, Address), (bool, Reserves));
        let success_result: SuccessResult =
            ((true, addresses.0), (true, addresses.1), (true, reserve_uints));
        let success_tokens = vec![
            Token::Tuple(vec![Token::Bool(true), Token::Address(addresses.0)]),
            Token::Tuple(vec![Token::Bool(true), Token::Address(addresses.1)]),
            Token::Tuple(vec![Token::Bool(true), Token::Tuple(reserves)]),
        ];
        type FailureResult = ((bool, String), (bool, String), (bool, String));
        let failure_result: FailureResult = (
            (false, error_message.clone()),
            (false, error_message.clone()),
            (false, error_message.clone()),
        );
        let failure_tokens = vec![
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
        ];

        // parse_errors

        let errors = parse_errors(success_tokens.clone());
        assert_eq!(errors.len(), 3);
        for e in errors {
            assert!(e.is_none());
        }

        let errors = parse_errors(failure_tokens.clone());
        assert_eq!(errors.len(), 3);
        for e in errors {
            assert_eq!(e.unwrap(), error_message.clone());
        }

        // parse_result

        let result = parse_result::<Provider<Http>, SuccessResult>(success_tokens.clone()).unwrap();
        assert_eq!(result.unwrap(), success_result);

        let result = parse_result::<Provider<Http>, FailureResult>(failure_tokens.clone()).unwrap();
        assert_eq!(result.unwrap(), failure_result);

        // parse_tokens_result

        let result = parse_tokens_result::<Provider<Http>>(success_tokens[0..2].to_vec()).unwrap();
        assert_eq!(result.unwrap(), addresses);

        let result = parse_tokens_result::<Provider<Http>>(failure_tokens.clone());
        assert!(result.unwrap().is_none());

        // parse_reserves_result

        let result = parse_reserves_result::<Provider<Http>>(success_tokens[2..].to_vec()).unwrap();
        assert_eq!(result.unwrap(), reserve_uints);

        let result = parse_reserves_result::<Provider<Http>>(failure_tokens);
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_sync() {
        let mut pair = default_pair();

        assert!(!pair.deployed());
        let tokens = pair.tokens().unwrap();
        assert_ne!(tokens.0, Address::zero());
        assert_ne!(tokens.1, Address::zero());

        pair.sync(true, false).await.unwrap();
        assert!(pair.deployed());
        assert!(pair.reserves().is_none());

        pair.sync(false, true).await.unwrap();
        let reserves = pair.reserves().unwrap();
        assert!(pair.deployed());
        assert_ne!(reserves.0, 0);
        assert_ne!(reserves.1, 0);
        assert_ne!(reserves.2, 0);
    }
}
