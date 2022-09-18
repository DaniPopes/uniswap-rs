use crate::{
    contracts::{address, try_address},
    v2::{Factory, Router, V2Protocol},
};
use ethers::{
    core::types::{Address, Chain, H256},
    providers::Middleware,
};
use std::fmt;

// TODO: Support for more chains

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

/// Represents a type of protocol that is, or is a fork of, Uniswap v2 or v3.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ProtocolType {
    /// UniswapV2, deployed on Ethereum.
    #[default]
    UniswapV2,

    /// UniswapV3, deployed on Ethereum, Polygon and Celo.
    UniswapV3,

    /// Sushiswap, deployed on most chains.
    Sushiswap,

    /// Pancakeswap, deployed only on Binance Smart Chain.
    Pancakeswap,

    /// Custom protocol.
    Custom {
        /// The protocol's factory address.
        factory: Address,

        /// The protocol's router address.
        router: Address,

        /// Whether the protocol is Uniswap v2 or v3.
        is_v2: bool,

        /// The codehash of the pair that the factory creates.
        pair_code_hash: Option<H256>,
    },
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ProtocolType {
    /// Instantiates a new custom protocol type.
    pub fn new(
        factory: Address,
        router: Address,
        is_v2: bool,
        pair_code_hash: Option<H256>,
    ) -> Self {
        Self::Custom { factory, router, is_v2, pair_code_hash }
    }

    /// Returns (factory_address, router_address), returning None if not found.
    pub fn try_addresses(&self, chain: Chain) -> (Option<Address>, Option<Address>) {
        if let Self::Custom { factory, router, .. } = self {
            (Some(*factory), Some(*router))
        } else {
            let (factory_name, router_name) = self.contract_names();
            (try_address(factory_name, chain), try_address(router_name, chain))
        }
    }

    /// Returns (factory_address, router_address), panicking if not found.
    pub fn addresses(&self, chain: Chain) -> (Address, Address) {
        if let Self::Custom { factory, router, .. } = self {
            (*factory, *router)
        } else {
            let (factory_name, router_name) = self.contract_names();
            (address(factory_name, chain), address(router_name, chain))
        }
    }

    /// Returns (factory_name, router_name).
    pub const fn contract_names(&self) -> (&str, &str) {
        match self {
            Self::UniswapV2 => ("UniswapV2Factory", "UniswapV2Router02"),
            Self::UniswapV3 => ("UniswapV3Factory", "UniswapV3Router02"),
            Self::Sushiswap => ("SushiswapV2Factory", "SushiswapV2Router02"),
            Self::Pancakeswap => ("PancakeFactory", "PancakeRouter"),
            Self::Custom { .. } => ("CustomFactory", "CustomRouter"),
        }
    }

    /// Returns the code hash of the pair created by the factory of the protocol.
    ///
    /// Returns None only when the variant is Custom and the pair_code_hash field is None.
    pub const fn pair_code_hash(&self) -> Option<H256> {
        match self {
            Self::UniswapV2 => Some(UNISWAP_V2_PAIR_CODE_HASH),
            Self::UniswapV3 => Some(UNISWAP_V3_POOL_CODE_HASH),
            Self::Sushiswap => Some(SUSHISWAP_PAIR_CODE_HASH),
            Self::Pancakeswap => Some(PANCAKESWAP_PAIR_CODE_HASH),
            Self::Custom { pair_code_hash, .. } => *pair_code_hash,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV2.
    pub const fn is_v2(&self) -> bool {
        match self {
            Self::Pancakeswap | Self::Sushiswap | Self::UniswapV2 => true,
            Self::UniswapV3 => false,
            Self::Custom { is_v2, .. } => *is_v2,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV3.
    pub const fn is_v3(&self) -> bool {
        !self.is_v2()
    }
}

/// Represents an automated market maker, a protocol that facilitates peer-to-peer market making and
/// swapping of ERC-20 tokens on the Ethereum blockchain.
#[derive(Debug)]
pub enum Protocol<M> {
    /// The UniswapV2 protocol.
    V2(V2Protocol<M>),

    /// The UniswapV3 protocol. WIP.
    V3,
}

impl<M> Clone for Protocol<M> {
    fn clone(&self) -> Self {
        match self {
            Self::V2(ref p) => Self::V2(p.clone()),
            Self::V3 => Self::V3,
        }
    }
}

impl<M: Middleware> Protocol<M> {
    /// Returns the factory.
    #[inline(always)]
    pub fn factory(&self) -> Factory {
        match self {
            Self::V2(p) => p.factory(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// Returns the router.
    #[inline(always)]
    pub fn router(&self) -> Router {
        match self {
            Self::V2(p) => p.router(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ProtocolType::*;

    #[test]
    fn test_versions() {
        let v2s = [UniswapV2, Sushiswap, Pancakeswap];
        let v3s = [UniswapV3];

        for v2 in v2s {
            assert!(v2.is_v2());
            assert!(!v2.is_v3());
        }

        for v3 in v3s {
            assert!(!v3.is_v2());
            assert!(v3.is_v3());
        }
    }
}
