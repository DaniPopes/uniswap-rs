use crate::contracts::{address, try_address};
use ethers::prelude::*;

/// [0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f](https://github.com/Uniswap/v2-periphery/blob/0335e8f7e1bd1e8d8329fd300aea2ef2f36dd19f/contracts/libraries/UniswapV2Library.sol#L24)
const UNISWAP_V2_PAIR_CODE_HASH: H256 = H256([
    0x96, 0xe8, 0xac, 0x42, 0x77, 0x19, 0x8f, 0xf8, 0xb6, 0xf7, 0x85, 0x47, 0x8a, 0xa9, 0xa3, 0x9f,
    0x40, 0x3c, 0xb7, 0x68, 0xdd, 0x02, 0xcb, 0xee, 0x32, 0x6c, 0x3e, 0x7d, 0xa3, 0x48, 0x84, 0x5f,
]);

/// [0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54](https://github.com/Uniswap/v3-periphery/blob/5bcdd9f67f9394f3159dad80d0dd01d37ca08c66/contracts/libraries/PoolAddress.sol#L6)
const UNISWAP_V3_POOL_CODE_HASH: H256 = H256([
    0xe3, 0x4f, 0x19, 0x9b, 0x19, 0xb2, 0xb4, 0xf4, 0x7f, 0x68, 0x44, 0x26, 0x19, 0xd5, 0x55, 0x52,
    0x7d, 0x24, 0x4f, 0x78, 0xa3, 0x29, 0x7e, 0xa8, 0x93, 0x25, 0xf8, 0x43, 0xf8, 0x7b, 0x8b, 0x54,
]);

/// [0xe18a34eb0e04b04f7a0ac29a6e80748dca96319b42c54d679cb821dca90c6303](https://github.com/sushiswap/sushiswap/blob/96eb88dee945c14c4bf90130c7f2e58e21c6f093/protocols/sushiswap/contracts/libraries/UniswapV2Library.sol#L26)
const SUSHISWAP_PAIR_CODE_HASH: H256 = H256([
    0xe1, 0x8a, 0x34, 0xeb, 0x0e, 0x04, 0xb0, 0x4f, 0x7a, 0x0a, 0xc2, 0x9a, 0x6e, 0x80, 0x74, 0x8d,
    0xca, 0x96, 0x31, 0x9b, 0x42, 0xc5, 0x4d, 0x67, 0x9c, 0xb8, 0x21, 0xdc, 0xa9, 0x0c, 0x63, 0x03,
]);

/// [0x00fb7f630766e6a796048ea87d01acd3068e8ff67d078148a3fa3f4a84f69bd5](https://bscscan.com/address/0xca143ce32fe78f1f7019d7d551a6402fc5350c73#readContract)
///
/// This is the on-chain hash as the one in [`pancakeswap/pancake-smart-contracts`](https://github.com/pancakeswap/pancake-smart-contracts/blob/d8f55093a43a7e8913f7730cfff3589a46f5c014/projects/exchange-protocol/contracts/libraries/PancakeLibrary.sol#L32)
/// is incorrect.
const PANCAKESWAP_PAIR_CODE_HASH: H256 = H256([
    0x00, 0xfb, 0x7f, 0x63, 0x07, 0x66, 0xe6, 0xa7, 0x96, 0x04, 0x8e, 0xa8, 0x7d, 0x01, 0xac, 0xd3,
    0x06, 0x8e, 0x8f, 0xf6, 0x7d, 0x07, 0x81, 0x48, 0xa3, 0xfa, 0x3f, 0x4a, 0x84, 0xf6, 0x9b, 0xd5,
]);

// TODO: Support for more chains
/// Represents a decentralized, on-chain ERC20 exchange protocol, like Uniswap.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Protocol {
    /// UniswapV2, deployed on Ethereum.
    #[default]
    UniswapV2,
    /// UniswapV3, deployed on Ethereum, Polygon and Celo.
    UniswapV3,
    /// Sushiswap, deployed on most chains.
    Sushiswap,
    /// Pancakeswap, deployed only on Binance Smart Chain.
    Pancakeswap,
    /// Custom protocol wrapping (router_address, factory_address). Not yet fully implemented.
    Custom(Address, Address),
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Protocol {
    /// Returns (router_address, factory_address), returning None on any error.
    pub fn try_addresses(&self, chain: Chain) -> (Option<Address>, Option<Address>) {
        if let Self::Custom(router_address, factory_address) = self {
            (Some(*router_address), Some(*factory_address))
        } else {
            let (router_name, factory_name) = self.contract_names();
            (try_address(router_name, chain), try_address(factory_name, chain))
        }
    }

    /// Returns (router_address, factory_address), panicking on any error.
    pub fn addresses(&self, chain: Chain) -> (Address, Address) {
        if let Self::Custom(router_address, factory_address) = self {
            (*router_address, *factory_address)
        } else {
            let (router_name, factory_name) = self.contract_names();
            (address(router_name, chain), address(factory_name, chain))
        }
    }

    /// Returns (router_name, factory_name).
    ///
    /// # Panics
    ///
    /// When called with the Custom variant.
    pub const fn contract_names(&self) -> (&str, &str) {
        match self {
            Self::UniswapV2 => ("UniswapV2Router02", "UniswapV2Factory"),
            Self::UniswapV3 => ("UniswapV3Router02", "UniswapV3Factory"),
            Self::Sushiswap => ("SushiswapV2Router02", "SushiswapV2Factory"),
            Self::Pancakeswap => ("PancakeRouter", "PancakeFactory"),
            Self::Custom(_, _) => panic!("not implemented for Protocol::Custom"),
        }
    }

    /// Returns the code hash of the pair created by the factory of the protocol.
    ///
    /// # Panics
    ///
    /// When called with the Custom variant.
    pub const fn get_pair_codehash(&self) -> H256 {
        match self {
            Self::UniswapV2 => UNISWAP_V2_PAIR_CODE_HASH,
            Self::UniswapV3 => UNISWAP_V3_POOL_CODE_HASH,
            Self::Sushiswap => SUSHISWAP_PAIR_CODE_HASH,
            Self::Pancakeswap => PANCAKESWAP_PAIR_CODE_HASH,
            // TODO
            Self::Custom(_, _) => panic!("not implemented for Protocol::Custom"),
        }
    }

    /// Returns whether the protocol is or is a fork of UniswapV2.
    pub fn is_v2(&self) -> bool {
        !self.is_v3()
    }

    /// Returns whether the protocol is or is a fork of UniswapV3.
    pub fn is_v3(&self) -> bool {
        matches!(self, Self::UniswapV3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Protocol::*;

    #[test]
    fn test_versions() {
        let v2s = [UniswapV2, Sushiswap, Pancakeswap];
        let v3s = [UniswapV3];

        for v2 in v2s {
            assert!(v2.is_v2());
            assert!(!v2.is_v3());
        }

        for v3 in v3s {
            assert!(v3.is_v3());
            assert!(!v3.is_v2());
        }
    }
}
