//! Contains all the relevant contract addresses. Modified from ethers/ethers-addressbook.

use ethers::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const CONTRACTS_JSON: &str = include_str!("./contracts.json");

static CONTRACTS_ADDRESS_BOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    /// Returns the address of the contract on the specified chain. If the contract's address is
    /// not found in the addressbook, the getter returns None.
    pub fn address(&self, chain: Chain) -> Option<Address> {
        self.addresses.get(&chain).cloned()
    }
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of
/// [contracts.json] we return None.
///
/// [contracts.json]: CONTRACTS_JSON
pub fn try_contract<S: Into<String>>(name: S) -> Option<Contract> {
    CONTRACTS_ADDRESS_BOOK.get(&name.into()).cloned()
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// [contracts.json] we return None.
///
/// [contracts.json]: CONTRACTS_JSON
pub fn try_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    let contracts = contract(&name.into());
    contracts.address(chain)
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of
/// [contracts.json] we panic.
///
/// [contracts.json]: CONTRACTS_JSON
pub fn contract<S: Into<String>>(name: S) -> Contract {
    let name = name.into();
    try_contract(&name).unwrap_or_else(|| panic!("Missing {} in contracts.json", name))
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// [contracts.json] we panic.
///
/// [contracts.json]: CONTRACTS_JSON
pub fn address<S: Into<String>>(name: S, chain: Chain) -> Address {
    let name = name.into();
    try_address(&name, chain)
        .unwrap_or_else(|| panic!("Missing {}[\"{}\"] in contracts.json", name, chain))
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
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::Mainnet)
            .is_some());
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::MoonbeamDev)
            .is_none());

        assert!(try_address("DAI", Chain::Mainnet).unwrap().is_some());
        assert!(try_address("DAI", Chain::MoonbeamDev).unwrap().is_none());
    }
}
