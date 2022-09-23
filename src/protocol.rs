use crate::{
    contracts::{address, try_address},
    errors::{FactoryResult, RouterError},
    v2::{Factory, Pair, Router, V2Protocol},
    Amount,
};
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

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
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProtocolType {
    /// UniswapV2, deployed on Ethereum.
    #[default]
    UniswapV2,

    /// Deployed on Ethereum, Polygon and Celo.
    UniswapV3,

    /// Deployed on most chains.
    Sushiswap,

    /// Deployed only on Binance Smart Chain.
    Pancakeswap,

    /// Deployed only on Polygon.
    Quickswap,

    /// Deployed only on Fantom.
    Spookyswap,

    /// Deployed only on Avalanche.
    Traderjoe,

    /// Custom protocol.
    Custom {
        /// The protocol's factory address.
        factory: Address,

        /// The protocol's router address.
        router: Address,

        /// Whether the protocol is Uniswap v2 or v3.
        is_v2: bool,

        /// The hash of the deployment code of the pair that the factory creates.
        pair_code_hash: H256,
    },
}

impl ProtocolType {
    /// Instantiates a new custom protocol type.
    pub fn new(factory: Address, router: Address, is_v2: bool, pair_code_hash: H256) -> Self {
        Self::Custom { factory, router, is_v2, pair_code_hash }
    }

    /// Returns all of the defined protocols.
    pub const fn all() -> [Self; 7] {
        use ProtocolType::*;
        [UniswapV2, UniswapV3, Sushiswap, Pancakeswap, Quickswap, Spookyswap, Traderjoe]
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
        use ProtocolType::*;
        match self {
            UniswapV2 => ("UniswapV2Factory", "UniswapV2Router02"),
            UniswapV3 => ("UniswapV3Factory", "UniswapV3Router02"),
            Sushiswap => ("SushiV2Factory", "SushiSwapRouter"),
            Pancakeswap => ("PancakeFactory", "PancakeRouter"),
            Quickswap => ("QuickFactory", "QuickRouter"),
            Spookyswap => ("SpookyFactory", "SpookyRouter"),
            Traderjoe => ("JoeFactory", "JoeRouter"),
            Custom { .. } => ("CustomFactory", "CustomRouter"),
        }
    }

    /// Returns the code hash of the pair created by the factory of the protocol.
    pub const fn pair_code_hash(&self) -> H256 {
        use ProtocolType::*;
        match self {
            UniswapV2 => UNISWAP_V2_PAIR_CODE_HASH,
            UniswapV3 => UNISWAP_V3_POOL_CODE_HASH,
            Sushiswap => SUSHISWAP_PAIR_CODE_HASH,
            Pancakeswap => PANCAKESWAP_PAIR_CODE_HASH,
            Quickswap => todo!(),  // QUICKSWAP_PAIR_CODE_HASH,
            Spookyswap => todo!(), // SPOOKYSWAP_PAIR_CODE_HASH,
            Traderjoe => todo!(),  // TRADERJOE_PAIR_CODE_HASH,
            Custom { pair_code_hash, .. } => *pair_code_hash,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV2.
    pub const fn is_v2(&self) -> bool {
        use ProtocolType::*;
        match self {
            Pancakeswap | Sushiswap | UniswapV2 | Quickswap | Spookyswap | Traderjoe => true,
            UniswapV3 => false,
            Custom { is_v2, .. } => *is_v2,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV3.
    pub const fn is_v3(&self) -> bool {
        !self.is_v2()
    }
}

/// Represents an automated market maker, a protocol that facilitates peer-to-peer market making and
/// swapping of ERC-20 tokens on the Ethereum blockchain.
#[derive(Clone, Debug)]
pub enum Protocol<M> {
    /// The UniswapV2 protocol.
    V2(V2Protocol<M>),

    /// The UniswapV3 protocol. WIP.
    V3,
}

impl<M: Middleware> Protocol<M> {
    /// Creates a new instance using the provided client, factory and tokens' addresses.
    pub fn new(client: Arc<M>, factory: Address, router: Address, protocol: ProtocolType) -> Self {
        match protocol {
            p if p.is_v2() => Self::V2(V2Protocol::new(client, factory, router, protocol)),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol \"{:?}\" is neither v2 nor v3", p),
        }
    }

    /// Creates a new instance using the provided chain.
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Self {
        match protocol {
            p if p.is_v2() => Self::V2(V2Protocol::new_with_chain(client, chain, protocol)),
            p if p.is_v3() => todo!("v3 is not yet implemented"),
            p => unreachable!("protocol \"{:?}\" is neither v2 nor v3", p),
        }
    }

    /// Returns a reference to the client.
    #[inline(always)]
    pub fn client(&self) -> Arc<M> {
        match self {
            Self::V2(p) => p.client(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /* ----------------------------------------- Factory ---------------------------------------- */

    /// Returns a reference to the factory.
    #[inline(always)]
    pub fn factory(&self) -> &Factory<M> {
        match self {
            Self::V2(p) => p.factory(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `pair_codehash` method.
    #[inline(always)]
    pub fn pair_codehash(&self) -> H256 {
        match self {
            Self::V2(p) => p.pair_codehash(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `create_pair` method.
    #[inline(always)]
    pub fn create_pair(&self, token_a: Address, token_b: Address) -> ContractCall<M, Address> {
        match self {
            Self::V2(p) => p.create_pair(token_a, token_b),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `pair_for` method.
    #[inline(always)]
    pub fn pair_for(&self, token_a: Address, token_b: Address) -> FactoryResult<Pair<M>, M> {
        match self {
            Self::V2(p) => p.pair_for(token_a, token_b),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /* ----------------------------------------- Router ----------------------------------------- */

    /// Returns a reference to the router.
    #[inline(always)]
    pub fn router(&self) -> &Router<M> {
        match self {
            Self::V2(p) => p.router(),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `add_liquidity` method.
    #[inline(always)]
    pub async fn add_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256, U256)>, RouterError<M>> {
        match self {
            Self::V2(p) => p.add_liquidity(
                token_a,
                token_b,
                amount_a_desired,
                amount_b_desired,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            ),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `remove_liquidity` method.
    #[inline(always)]
    pub async fn remove_liquidity(
        &self,
        token_a: Address,
        token_b: Address,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Address,
        deadline: U256,
    ) -> Result<ContractCall<M, (U256, U256)>, RouterError<M>> {
        match self {
            Self::V2(p) => p.remove_liquidity(
                token_a,
                token_b,
                liquidity,
                amount_a_min,
                amount_b_min,
                to,
                deadline,
            ),
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }

    /// The protocol's `swap` method.
    #[inline(always)]
    pub async fn swap(
        &self,
        amount: Amount,
        slippage_tolerance: f32,
        path: &[Address],
        to: Address,
        deadline: U256,
        weth: Address,
    ) -> Result<ContractCall<M, Vec<U256>>, RouterError<M>> {
        match self {
            Self::V2(p) => p.swap(amount, slippage_tolerance, path, to, deadline, weth).await,
            Self::V3 => todo!("v3 is not yet implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Chain::*;
    use ProtocolType::*;

    #[test]
    fn test_versions() {
        let v2s = [UniswapV2, Sushiswap, Pancakeswap, Quickswap, Spookyswap, Traderjoe];
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

    #[test]
    fn test_addresses() {
        let protocols = ProtocolType::all();

        let mainnet = || vec![Mainnet];
        let eth_t = || vec![Rinkeby, Ropsten, Goerli, Kovan];
        let polygon = || vec![Polygon, PolygonMumbai];
        let l2 = || vec![Optimism, Arbitrum];
        let l2_t = || vec![OptimismGoerli, OptimismKovan, ArbitrumTestnet];
        let bsc = || vec![BinanceSmartChain, BinanceSmartChainTestnet];
        let avax = || vec![Avalanche, AvalancheFuji];

        #[rustfmt::skip]
        let chains = vec![
            /*  UniswapV2  */ vec![mainnet(), eth_t()].concat(),
            /*  UniswapV3  */ vec![mainnet(), eth_t(), polygon(), l2(), l2_t()].concat(),
            /*  Sushiswap  */ vec![vec![Fantom, Moonriver, Moonbeam, XDai], mainnet(), polygon(), bsc(), avax()].concat(),
            /* Pancakeswap */ bsc(),
            /*  Quickswap  */ polygon(),
            /*  Spookyswap */ vec![Fantom],
            /*  Traderjoe  */ avax(),
        ];

        assert_eq!(protocols.len(), chains.len());

        for (protocol, p_chains) in protocols.iter().zip(chains) {
            for chain in p_chains {
                let addresses = protocol.addresses(chain);
                assert_ne!(addresses.0, Address::zero());
                assert_ne!(addresses.1, Address::zero());
            }
        }
    }
}
