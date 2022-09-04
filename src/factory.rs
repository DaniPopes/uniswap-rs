use crate::Protocol;
use ethers::prelude::*;

/// [0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f](https://github.com/Uniswap/v2-periphery/blob/0335e8f7e1bd1e8d8329fd300aea2ef2f36dd19f/contracts/libraries/UniswapV2Library.sol#L24)
pub const UNISWAP_V2_PAIR_CODE_HASH: H256 = H256([
    0x96, 0xe8, 0xac, 0x42, 0x77, 0x19, 0x8f, 0xf8, 0xb6, 0xf7, 0x85, 0x47, 0x8a, 0xa9, 0xa3, 0x9f,
    0x40, 0x3c, 0xb7, 0x68, 0xdd, 0x02, 0xcb, 0xee, 0x32, 0x6c, 0x3e, 0x7d, 0xa3, 0x48, 0x84, 0x5f,
]);

/// [0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54](https://github.com/Uniswap/v3-periphery/blob/5bcdd9f67f9394f3159dad80d0dd01d37ca08c66/contracts/libraries/PoolAddress.sol#L6)
pub const UNISWAP_V3_POOL_CODE_HASH: H256 = H256([
    0xe3, 0x4f, 0x19, 0x9b, 0x19, 0xb2, 0xb4, 0xf4, 0x7f, 0x68, 0x44, 0x26, 0x19, 0xd5, 0x55, 0x52,
    0x7d, 0x24, 0x4f, 0x78, 0xa3, 0x29, 0x7e, 0xa8, 0x93, 0x25, 0xf8, 0x43, 0xf8, 0x7b, 0x8b, 0x54,
]);

/// [0xe18a34eb0e04b04f7a0ac29a6e80748dca96319b42c54d679cb821dca90c6303](https://github.com/sushiswap/sushiswap/blob/96eb88dee945c14c4bf90130c7f2e58e21c6f093/protocols/sushiswap/contracts/libraries/UniswapV2Library.sol#L26)
pub const SUSHISWAP_PAIR_CODE_HASH: H256 = H256([
    0xe1, 0x8a, 0x34, 0xeb, 0x0e, 0x04, 0xb0, 0x4f, 0x7a, 0x0a, 0xc2, 0x9a, 0x6e, 0x80, 0x74, 0x8d,
    0xca, 0x96, 0x31, 0x9b, 0x42, 0xc5, 0x4d, 0x67, 0x9c, 0xb8, 0x21, 0xdc, 0xa9, 0x0c, 0x63, 0x03,
]);

/// Represents a Uniswap factory.
#[derive(Clone, Copy, Debug)]
pub struct Factory {
    address: Address,
    protocol: Protocol,
}

impl From<Factory> for Address {
    fn from(factory: Factory) -> Self {
        factory.address
    }
}

impl Factory {
    /// Creates a new instance of a Uniswap factory.
    /// Note: either an address or a chain must be provided.
    pub fn new(address: Option<Address>, chain: Option<Chain>, protocol: Protocol) -> Self {
        let address = address.unwrap_or_else(|| {
            protocol
                .get_addresses(
                    chain.expect("Factory::new: Must provide at least one of: address or chain"),
                )
                .1
        });

        Self { address, protocol }
    }

    /// Returns the contract address of the factory.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the protocol of the factory.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Returns the codehash of the pair that this factory deploys.
    pub fn pair_codehash(&self) -> H256 {
        match self.protocol {
            Protocol::UniswapV2 => UNISWAP_V2_PAIR_CODE_HASH,
            Protocol::UniswapV3 => UNISWAP_V3_POOL_CODE_HASH,
            Protocol::Sushiswap => SUSHISWAP_PAIR_CODE_HASH,
            Protocol::Custom(_router, _factory) => {
                todo!("Factory::pair_codehash of Protocol::Custom is not yet implemented")
            }
        }
    }
}
