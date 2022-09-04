use crate::contracts::address;
use ethers::prelude::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Protocol {
    #[default]
    UniswapV2,
    UniswapV3,
    Sushiswap,
    Custom(Address, Address),
}

impl Protocol {
    /// Returns (router_address, factory_address)
    pub fn get_addresses(&self, chain: Chain) -> (Address, Address) {
        if let Self::Custom(router_address, factory_address) = self {
            (*router_address, *factory_address)
        } else {
            let (router_name, factory_name) = match self {
                Self::UniswapV2 => ("UniswapV2Router02", "UniswapV2Factory"),
                Self::UniswapV3 => ("UniswapV3Router02", "UniswapV3Factory"),
                Self::Sushiswap => ("SushiswapV2Router02", "SushiswapV2Factory"),
                Self::Custom(_, _) => unreachable!(),
            };

            (address(router_name, chain), address(factory_name, chain))
        }
    }
}
