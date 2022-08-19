use ethers::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

// modified from ethers/ethers-addressbook/src/lib.rs

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
/// [ethers-addressbook] we return None.
///
/// [ethers-addressbook]: https://github.com/gakonst/ethers-rs/tree/master/ethers-addressbook
pub fn try_contract<S: Into<String>>(name: S) -> Option<Contract> {
    CONTRACTS_ADDRESS_BOOK.get(&name.into()).cloned()
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of
/// [ethers-addressbook] we panic.
///
/// [ethers-addressbook]: https://github.com/gakonst/ethers-rs/tree/master/ethers-addressbook
pub fn contract<S: Into<String>>(name: S) -> Contract {
    let name: String = name.into();
    try_contract(&name).unwrap_or_else(|| panic!("Missing {} in contracts.json", name))
}

/// Fetch the address from a contract's addressbook. If the contract name is not a part of
/// [ethers-addressbook] we return None.
///
/// [ethers-addressbook]: https://github.com/gakonst/ethers-rs/tree/master/ethers-addressbook
pub fn try_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    let name: String = name.into();
    let contracts = contract(&name);
    contracts.address(chain)
}

/// Fetch the address from a contract's addressbook. If the contract name is not a part of
/// [ethers-addressbook] we panic.
///
/// [ethers-addressbook]: https://github.com/gakonst/ethers-rs/tree/master/ethers-addressbook
pub fn address<S: Into<String>>(name: S, chain: Chain) -> Address {
    let name: String = name.into();
    let contracts = contract(&name);
    contracts
        .address(chain)
        .unwrap_or_else(|| panic!("Missing {}[\"{}\"] in contracts.json", name, chain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        assert!(try_contract("DAI").is_some());
        assert!(try_contract("USDC").is_some());
        assert!(try_contract("rand").is_none());
    }

    #[test]
    fn test_addrs() {
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::Mainnet)
            .is_some());
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::MoonbeamDev)
            .is_none());
    }
}
