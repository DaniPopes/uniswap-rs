//! Contains many useful smart contract addresses. Modified from [`addressbook`].
//!
//! [`addressbook`]: https://github.com/gakonst/ethers-rs/blob/master/ethers-addressbook
//!
//! ## Definitions
//!
//! - `ETH` stores the ERC20 token that represents the **Ethereum token** in the correspoding chain.
//!   For example, on Mainnet this is the [`WETH`] token, while on BSC it's [`Binance-Peg Ethereum
//!   Token`].
//! - `WETH` stores the ERC20-wrapped **native token**, which is not necessarily Ethereum as the
//!   name implies. This is done for easier access as most UniswapV2 router forks call this token as
//!   "WETH" even if their native token is not Ethereum. For example, on Mainnet this is still
//!   [`WETH`], while on BSC it's [`WBNB`].
//!
//! [`WETH`]: https://etherscan.io/token/0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
//! [`Binance-Peg Ethereum Token`]: https://bscscan.com/token/0x2170ed0880ac9a755fd29b2688956bd959f933f8
//! [`WBNB`]:  https://bscscan.com/token/0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c
//!
//! ## Sources
//!
//! - UniswapV2
//!   - Factory: <https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory>
//!   - Router: <https://docs.uniswap.org/protocol/V2/reference/smart-contracts/router-02>
//! - UniswapV3: <https://docs.uniswap.org/protocol/reference/deployments>
//! - Sushiswap: <https://docs.sushi.com/docs/Developers/Deployment%20Addresses>
//! - Quickswap
//!   - Factory: <https://docs.quickswap.exchange/reference/smart-contracts/01-factory>
//!   - Router: <https://docs.quickswap.exchange/reference/smart-contracts/router02>
//! - Spookyswap: <https://docs.spooky.fi/Resources/contracts>
//! - Traderjoe: <https://docs.traderjoexyz.com/en/security-and-contracts/contracts>

use ethers::prelude::*;
use serde::Deserialize;
use std::{borrow::Borrow, collections::HashMap};

const CONTRACTS_JSON: &str = include_str!("./contracts.json");

static CONTRACTS_ADDRESS_BOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    /// Returns the address of the contract on the specified chain. Returns None if the contract's
    /// address is not found in the addressbook.
    pub fn address<C: Borrow<Chain>>(&self, chain: C) -> Option<Address> {
        self.addresses.get(chain.borrow()).copied()
    }
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we return None.
pub fn try_contract<S: Borrow<str>>(name: S) -> Option<Contract> {
    CONTRACTS_ADDRESS_BOOK.get(name.borrow()).cloned()
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we return None.
pub fn try_address<S: Borrow<str>, C: Borrow<Chain>>(name: S, chain: C) -> Option<Address> {
    let contract = try_contract(name);
    contract.and_then(|contract| contract.address(chain))
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we panic.
pub fn contract<S: Borrow<str>>(name: S) -> Contract {
    let name = name.borrow();
    try_contract(name).unwrap_or_else(|| {
        panic!("uniswap_rs::contracts: \"{name}\" is not present in addressbook")
    })
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we panic.
pub fn address<S: Borrow<str>, C: Borrow<Chain>>(name: S, chain: C) -> Address {
    let name = name.borrow();
    let chain = chain.borrow();
    let contract = contract(name);
    contract.address(chain).unwrap_or_else(|| {
        panic!("uniswap_rs::contracts: Chain \"{chain:?}\" for contract \"{name}\" is not present in addressbook")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contracts() {
        assert!(try_contract("DAI").is_some());
        assert!(try_contract("USDC").is_some());
        assert!(try_contract("rand").is_none());
    }

    #[test]
    fn test_addresses() {
        assert!(try_contract("DAI").unwrap().address(Chain::Mainnet).is_some());
        assert!(try_contract("DAI").unwrap().address(Chain::MoonbeamDev).is_none());

        assert!(try_address("DAI", Chain::Mainnet).is_some());
        assert!(try_address("DAI", Chain::MoonbeamDev).is_none());
    }
}
