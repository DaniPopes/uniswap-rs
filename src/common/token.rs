use crate::{contracts::bindings::ierc20::IERC20, errors::Result};
use ethers_contract::{builders::ContractCall, Multicall};
use ethers_core::{
    abi::Tokenizable,
    types::{Address, Chain},
};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

const UNKNOWN: &str = "unknown";

contract_struct! {
    /// An ERC20 token.
    pub struct Erc20<M> {
        /// The token's contract.
        contract: IERC20<M>,

        /// The token's name.
        pub name: Option<String>,

        /// The token's symbol.
        pub symbol: Option<String>,

        /// The token's decimals.
        pub decimals: Option<u8>,
    }
}

impl<M> fmt::Display for Erc20<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        let symbol = self.symbol();
        f.write_fmt(format_args!("{name} ({symbol})"))?;
        if f.alternate() {
            let address = self.contract.address();
            f.write_fmt(format_args!(" @ {address}"))?;
        }
        Ok(())
    }
}

impl<M> Erc20<M> {
    /// The token's name.
    ///
    /// Defaults to "unknown" if not synced or not present.
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or(UNKNOWN)
    }

    /// The token's symbol.
    ///
    /// Defaults to "unknown" if not synced or not present.
    pub fn symbol(&self) -> &str {
        self.symbol.as_deref().unwrap_or(UNKNOWN)
    }

    /// The token's decimals.
    ///
    /// Defaults to `18` if not synced or not present.
    pub fn decimals(&self) -> u8 {
        self.decimals.unwrap_or(18)
    }
}

impl<M: Middleware> Erc20<M> {
    /// Creates a new, empty token.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IERC20::new(address, client);
        Self { contract, name: None, symbol: None, decimals: None }
    }

    /// Creates a new token with the provided metadata.
    pub fn new_with_metadata(
        client: Arc<M>,
        address: Address,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Self {
        let contract = IERC20::new(address, client);
        Self { contract, name: Some(name), symbol: Some(symbol), decimals: Some(decimals) }
    }

    /// Returns the contract calls for fetching the token's name, symbol and decimals.
    pub fn metadata(
        &self,
    ) -> (ContractCall<M, String>, ContractCall<M, String>, ContractCall<M, u8>) {
        (self.contract.name(), self.contract.symbol(), self.contract.decimals())
    }

    /// Adds the getter calls to the provided [Multicall].
    pub fn add_metadata<'m>(&self, multicall: &'m mut Multicall<M>) -> &'m mut Multicall<M> {
        let (name, symbol, decimals) = self.metadata();
        multicall.add_call(name, true).add_call(symbol, true).add_call(decimals, true)
    }

    /// Syncs the token's name, symbol and decimals.
    pub async fn sync(&mut self, chain: Chain) -> Result<&mut Self> {
        let mut multicall = Multicall::new_with_chain_id(self.client(), None, Some(chain))?;
        self.add_metadata(&mut multicall);

        // TODO: results
        #[allow(clippy::single_match)]
        match multicall.call_raw().await {
            Ok(tokens) => {
                let mut tokens = tokens.into_iter();
                macro_rules! assign_if_ok {
                    ($($var:ident)+) => {$(
                        if let Some(token) = tokens.next() {
                            if let Ok((_, var)) = <(bool, _)>::from_token(token) {
                                self.$var = Some(var);
                            }
                        }
                    )+};
                }
                assign_if_ok!(name symbol decimals);
            }
            Err(_) => { /* TODO */ }
        }

        Ok(self)
    }
}

#[cfg(all(test, feature = "addresses"))]
mod tests {
    use super::*;
    use ethers_providers::{Http, Provider, MAINNET};

    fn default_token() -> Erc20<Provider<Http>> {
        let address = crate::contracts::addresses::address("WETH", Chain::Mainnet);
        let provider = Arc::new(MAINNET.provider());
        Erc20::new(provider, address)
    }

    #[test]
    fn test_fmt() {
        let mut token = default_token();
        token.name = Some("Wrapped Ether".into());
        token.symbol = Some("WETH".into());

        assert_eq!(format!("{token}"), "Wrapped Ether (WETH)");
        assert_eq!(format!("{token:#}"), "Wrapped Ether (WETH) @ 0xc02a…6cc2");
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn metadata() {
        let mut token = default_token();
        token.sync(Chain::Mainnet).await.unwrap();

        assert_eq!(token.name.unwrap(), "Wrapped Ether");
        assert_eq!(token.symbol.unwrap(), "WETH");
        assert_eq!(token.decimals.unwrap(), 18);
    }
}
