use super::pair_code_hashes::*;
use crate::contracts::{address, try_address};
use ethers::prelude::*;

/// Represents a type of protocol that is, or is a fork of, Uniswap v2 or v3.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProtocolType {
    /// Deployed on Ethereum and its testnets.
    #[default]
    UniswapV2,

    /// Deployed on Ethereum, Polygon, Celo and testnets.
    UniswapV3,

    /// Deployed on most chains.
    Sushiswap,

    /// Deployed only on Binance Smart Chain and its testnet.
    Pancakeswap,

    /// Deployed only on Polygon and its Mumbai testnet.
    Quickswap,

    /// Deployed only on Fantom and its testnet.
    Spookyswap,

    /// Deployed only on Avalanche and its Fuji testnet.
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
    ///
    /// Note: `chain` is used only when the pair code hash differs in the same protocol, for example
    /// `Pancakeswap` has two different code hashes for BSC mainnet and testnet.
    pub fn pair_code_hash(&self, chain: Option<Chain>) -> H256 {
        use ProtocolType::*;
        match self {
            UniswapV2 => UNISWAP_V2_PAIR_CODE_HASH,
            UniswapV3 => UNISWAP_V3_POOL_CODE_HASH,
            Sushiswap => SUSHISWAP_PAIR_CODE_HASH,
            Pancakeswap => chain_or(
                chain,
                Chain::BinanceSmartChainTestnet,
                PANCAKESWAP_PAIR_CODE_HASH,
                PANCAKESWAP_TESTNET_PAIR_CODE_HASH,
            ),
            Quickswap => QUICKSWAP_PAIR_CODE_HASH,
            Spookyswap => chain_or(
                chain,
                Chain::FantomTestnet,
                SPOOKYSWAP_PAIR_CODE_HASH,
                SPOOKYSWAP_TESTNET_PAIR_CODE_HASH,
            ),
            Traderjoe => chain_or(
                chain,
                Chain::AvalancheFuji,
                TRADERJOE_PAIR_CODE_HASH,
                TRADERJOE_TESTNET_PAIR_CODE_HASH,
            ),
            Custom { pair_code_hash, .. } => *pair_code_hash,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV2.
    pub const fn is_v2(&self) -> bool {
        use ProtocolType::*;
        match self {
            UniswapV2 | Sushiswap | Pancakeswap | Quickswap | Spookyswap | Traderjoe => true,
            UniswapV3 => false,
            Custom { is_v2, .. } => *is_v2,
        }
    }

    /// Returns whether the protocol is, or is a fork of, UniswapV3.
    pub const fn is_v3(&self) -> bool {
        !self.is_v2()
    }
}

/// if c1 == c2 {h2} else {h1}
fn chain_or(c1: Option<Chain>, c2: Chain, h1: H256, h2: H256) -> H256 {
    if let Some(c1) = c1 {
        if c1 == c2 {
            return h2
        }
    }
    h1
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

        let mainnet = || vec![Mainnet, Rinkeby, Ropsten, Goerli, Kovan];
        let l2 = || vec![Optimism, Arbitrum];
        let l2_t = || vec![OptimismGoerli, OptimismKovan, ArbitrumTestnet];
        let bsc = || vec![BinanceSmartChain, BinanceSmartChainTestnet];
        let polygon = || vec![Polygon, PolygonMumbai];
        let avax = || vec![Avalanche, AvalancheFuji];

        #[rustfmt::skip]
        let chains = vec![
            /*  UniswapV2   */ mainnet(),
            /*  UniswapV3   */ vec![mainnet(), polygon(), l2(), l2_t()].concat(),
            /*  Sushiswap   */ vec![vec![Fantom, Moonriver, Moonbeam, XDai], mainnet(), polygon(), bsc(), avax()].concat(),
            /* Pancakeswap  */ bsc(),
            /*  Quickswap   */ polygon(),
            /*  Spookyswap  */ vec![Fantom, FantomTestnet],
            /*  Traderjoe   */ avax(),
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
