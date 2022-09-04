pub use i_uniswap_v3_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_pool {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    #[doc = "IUniswapV3Pool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV3POOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Burn\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"Collect\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"CollectProtocol\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"paid0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"paid1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Flash\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint16\",\n                \"name\": \"observationCardinalityNextOld\",\n                \"type\": \"uint16\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint16\",\n                \"name\": \"observationCardinalityNextNew\",\n                \"type\": \"uint16\"\n            }\n        ],\n        \"name\": \"IncreaseObservationCardinalityNext\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint160\",\n                \"name\": \"sqrtPriceX96\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"int24\",\n                \"name\": \"tick\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"name\": \"Initialize\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"amount\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Mint\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol0Old\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol1Old\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol0New\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol1New\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"name\": \"SetFeeProtocol\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"int256\",\n                \"name\": \"amount0\",\n                \"type\": \"int256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"int256\",\n                \"name\": \"amount1\",\n                \"type\": \"int256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint160\",\n                \"name\": \"sqrtPriceX96\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint128\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"int24\",\n                \"name\": \"tick\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"name\": \"Swap\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"burn\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0Requested\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1Requested\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"collect\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0Requested\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1Requested\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"collectProtocol\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"factory\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"fee\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint24\",\n                \"name\": \"\",\n                \"type\": \"uint24\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeGrowthGlobal0X128\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeGrowthGlobal1X128\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"flash\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint16\",\n                \"name\": \"observationCardinalityNext\",\n                \"type\": \"uint16\"\n            }\n        ],\n        \"name\": \"increaseObservationCardinalityNext\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"sqrtPriceX96\",\n                \"type\": \"uint160\"\n            }\n        ],\n        \"name\": \"initialize\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"liquidity\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"maxLiquidityPerTick\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"mint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"observations\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"blockTimestamp\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"int56\",\n                \"name\": \"tickCumulative\",\n                \"type\": \"int56\"\n            },\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"secondsPerLiquidityCumulativeX128\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"initialized\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32[]\",\n                \"name\": \"secondsAgos\",\n                \"type\": \"uint32[]\"\n            }\n        ],\n        \"name\": \"observe\",\n        \"outputs\": [\n            {\n                \"internalType\": \"int56[]\",\n                \"name\": \"tickCumulatives\",\n                \"type\": \"int56[]\"\n            },\n            {\n                \"internalType\": \"uint160[]\",\n                \"name\": \"secondsPerLiquidityCumulativeX128s\",\n                \"type\": \"uint160[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"key\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"positions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"_liquidity\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"feeGrowthInside0LastX128\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"feeGrowthInside1LastX128\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"tokensOwed0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"tokensOwed1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"protocolFees\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"token0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"token1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol0\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol1\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"name\": \"setFeeProtocol\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"slot0\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"sqrtPriceX96\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tick\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"uint16\",\n                \"name\": \"observationIndex\",\n                \"type\": \"uint16\"\n            },\n            {\n                \"internalType\": \"uint16\",\n                \"name\": \"observationCardinality\",\n                \"type\": \"uint16\"\n            },\n            {\n                \"internalType\": \"uint16\",\n                \"name\": \"observationCardinalityNext\",\n                \"type\": \"uint16\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"feeProtocol\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"unlocked\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"name\": \"snapshotCumulativesInside\",\n        \"outputs\": [\n            {\n                \"internalType\": \"int56\",\n                \"name\": \"tickCumulativeInside\",\n                \"type\": \"int56\"\n            },\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"secondsPerLiquidityInsideX128\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"secondsInside\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"zeroForOne\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"int256\",\n                \"name\": \"amountSpecified\",\n                \"type\": \"int256\"\n            },\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"sqrtPriceLimitX96\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"int256\",\n                \"name\": \"amount0\",\n                \"type\": \"int256\"\n            },\n            {\n                \"internalType\": \"int256\",\n                \"name\": \"amount1\",\n                \"type\": \"int256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"int16\",\n                \"name\": \"wordPosition\",\n                \"type\": \"int16\"\n            }\n        ],\n        \"name\": \"tickBitmap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"tickSpacing\",\n        \"outputs\": [\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tick\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"name\": \"ticks\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"liquidityGross\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"int128\",\n                \"name\": \"liquidityNet\",\n                \"type\": \"int128\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"feeGrowthOutside0X128\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"feeGrowthOutside1X128\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"int56\",\n                \"name\": \"tickCumulativeOutside\",\n                \"type\": \"int56\"\n            },\n            {\n                \"internalType\": \"uint160\",\n                \"name\": \"secondsPerLiquidityOutsideX128\",\n                \"type\": \"uint160\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"secondsOutside\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"initialized\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"token0\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"token1\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    pub struct IUniswapV3Pool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV3Pool<M> {
        fn clone(&self) -> Self {
            IUniswapV3Pool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3Pool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IUniswapV3Pool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3Pool)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> IUniswapV3Pool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IUNISWAPV3POOL_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `burn` (0xa34123a7) function"]
        pub fn burn(
            &self,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 65, 35, 167], (tick_lower, tick_upper, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collect` (0x4f1eb3d8) function"]
        pub fn collect(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [79, 30, 179, 216],
                    (recipient, tick_lower, tick_upper, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collectProtocol` (0x85b66729) function"]
        pub fn collect_protocol(
            &self,
            recipient: ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fee` (0xddca3f43) function"]
        pub fn fee(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeGrowthGlobal0X128` (0xf3058399) function"]
        pub fn fee_growth_global_0x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([243, 5, 131, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeGrowthGlobal1X128` (0x46141319) function"]
        pub fn fee_growth_global_1x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([70, 20, 19, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flash` (0x490e6cbc) function"]
        pub fn flash(
            &self,
            recipient: ethers::core::types::Address,
            amount_0: ethers::core::types::U256,
            amount_1: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 14, 108, 188], (recipient, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function"]
        pub fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], observation_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf637731d) function"]
        pub fn initialize(
            &self,
            sqrt_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 55, 115, 29], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidity` (0x1a686502) function"]
        pub fn liquidity(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function"]
        pub fn max_liquidity_per_tick(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x3c8a7d8d) function"]
        pub fn mint(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([60, 138, 125, 141], (recipient, tick_lower, tick_upper, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `observations` (0x252c09d7) function"]
        pub fn observations(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u32, i64, ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash([37, 44, 9, 215], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `observe` (0x883bdbfd) function"]
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i64>, ::std::vec::Vec<ethers::core::types::U256>),
        > {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positions` (0x514ea4bf) function"]
        pub fn positions(
            &self,
            key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (u128, ethers::core::types::U256, ethers::core::types::U256, u128, u128),
        > {
            self.0
                .method_hash([81, 78, 164, 191], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolFees` (0x1ad8b03b) function"]
        pub fn protocol_fees(&self) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeProtocol` (0x8206a4d1) function"]
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slot0` (0x3850c7bd) function"]
        pub fn slot_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, i32, u16, u16, u16, u8, bool),
        > {
            self.0
                .method_hash([56, 80, 199, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `snapshotCumulativesInside` (0xa38807f2) function"]
        pub fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, (i64, ethers::core::types::U256, u32)>
        {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x128acb08) function"]
        pub fn swap(
            &self,
            recipient: ethers::core::types::Address,
            zero_for_one: bool,
            amount_specified: I256,
            sqrt_price_limit_x96: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, (I256, I256)> {
            self.0
                .method_hash(
                    [18, 138, 203, 8],
                    (recipient, zero_for_one, amount_specified, sqrt_price_limit_x96, data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickBitmap` (0x5339c296) function"]
        pub fn tick_bitmap(
            &self,
            word_position: i16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([83, 57, 194, 150], word_position)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickSpacing` (0xd0c93a7c) function"]
        pub fn tick_spacing(&self) -> ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ticks` (0xf30dba93) function"]
        pub fn ticks(
            &self,
            tick: i32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                i128,
                ethers::core::types::U256,
                ethers::core::types::U256,
                i64,
                ethers::core::types::U256,
                u32,
                bool,
            ),
        > {
            self.0
                .method_hash([243, 13, 186, 147], tick)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Collect` event"]
        pub fn collect_filter(&self) -> ethers::contract::builders::Event<M, CollectFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollectProtocol` event"]
        pub fn collect_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollectProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Flash` event"]
        pub fn flash_filter(&self) -> ethers::contract::builders::Event<M, FlashFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseObservationCardinalityNext` event"]
        pub fn increase_observation_cardinality_next_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseObservationCardinalityNextFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialize` event"]
        pub fn initialize_filter(&self) -> ethers::contract::builders::Event<M, InitializeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetFeeProtocol` event"]
        pub fn set_fee_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetFeeProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IUniswapV3PoolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IUniswapV3Pool<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,int24,int24,uint128,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Collect", abi = "Collect(address,address,int24,int24,uint128,uint128)")]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "CollectProtocol", abi = "CollectProtocol(address,address,uint128,uint128)")]
    pub struct CollectProtocolFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Flash", abi = "Flash(address,address,uint256,uint256,uint256,uint256)")]
    pub struct FlashFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub paid_0: ethers::core::types::U256,
        pub paid_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "IncreaseObservationCardinalityNext",
        abi = "IncreaseObservationCardinalityNext(uint16,uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextFilter {
        pub observation_cardinality_next_old: u16,
        pub observation_cardinality_next_new: u16,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Initialize", abi = "Initialize(uint160,int24)")]
    pub struct InitializeFilter {
        pub sqrt_price_x96: ethers::core::types::U256,
        pub tick: i32,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Mint", abi = "Mint(address,address,int24,int24,uint128,uint256,uint256)")]
    pub struct MintFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "SetFeeProtocol", abi = "SetFeeProtocol(uint8,uint8,uint8,uint8)")]
    pub struct SetFeeProtocolFilter {
        pub fee_protocol_0_old: u8,
        pub fee_protocol_1_old: u8,
        pub fee_protocol_0_new: u8,
        pub fee_protocol_1_new: u8,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Swap", abi = "Swap(address,address,int256,int256,uint160,uint128,int24)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: I256,
        pub amount_1: I256,
        pub sqrt_price_x96: ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolEvents {
        BurnFilter(BurnFilter),
        CollectFilter(CollectFilter),
        CollectProtocolFilter(CollectProtocolFilter),
        FlashFilter(FlashFilter),
        IncreaseObservationCardinalityNextFilter(IncreaseObservationCardinalityNextFilter),
        InitializeFilter(InitializeFilter),
        MintFilter(MintFilter),
        SetFeeProtocolFilter(SetFeeProtocolFilter),
        SwapFilter(SwapFilter),
    }
    impl ethers::contract::EthLogDecode for IUniswapV3PoolEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::BurnFilter(decoded))
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::CollectFilter(decoded))
            }
            if let Ok(decoded) = CollectProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::CollectProtocolFilter(decoded))
            }
            if let Ok(decoded) = FlashFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::FlashFilter(decoded))
            }
            if let Ok(decoded) = IncreaseObservationCardinalityNextFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::IncreaseObservationCardinalityNextFilter(decoded))
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::InitializeFilter(decoded))
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::MintFilter(decoded))
            }
            if let Ok(decoded) = SetFeeProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::SetFeeProtocolFilter(decoded))
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::SwapFilter(decoded))
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolEvents::BurnFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::CollectFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::CollectProtocolFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::FlashFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::IncreaseObservationCardinalityNextFilter(element) => {
                    element.fmt(f)
                }
                IUniswapV3PoolEvents::InitializeFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::MintFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::SetFeeProtocolFilter(element) => element.fmt(f),
                IUniswapV3PoolEvents::SwapFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(int24,int24,uint128)` and selector `[163, 65, 35, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(int24,int24,uint128)")]
    pub struct BurnCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `[79, 30, 179, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collect", abi = "collect(address,int24,int24,uint128,uint128)")]
    pub struct CollectCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `[133, 182, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collectProtocol", abi = "collectProtocol(address,uint128,uint128)")]
    pub struct CollectProtocolCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `fee` function with signature `fee()` and selector `[221, 202, 63, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    #[doc = "Container type for all input parameters for the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `[243, 5, 131, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeGrowthGlobal0X128", abi = "feeGrowthGlobal0X128()")]
    pub struct FeeGrowthGlobal0X128Call;
    #[doc = "Container type for all input parameters for the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `[70, 20, 19, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeGrowthGlobal1X128", abi = "feeGrowthGlobal1X128()")]
    pub struct FeeGrowthGlobal1X128Call;
    #[doc = "Container type for all input parameters for the `flash` function with signature `flash(address,uint256,uint256,bytes)` and selector `[73, 14, 108, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "flash", abi = "flash(address,uint256,uint256,bytes)")]
    pub struct FlashCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `increaseObservationCardinalityNext` function with signature `increaseObservationCardinalityNext(uint16)` and selector `[50, 20, 143, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub observation_cardinality_next: u16,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint160)` and selector `[246, 55, 115, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint160)")]
    pub struct InitializeCall {
        pub sqrt_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidity` function with signature `liquidity()` and selector `[26, 104, 101, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
    #[doc = "Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `[60, 138, 125, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128,bytes)")]
    pub struct MintCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `observations` function with signature `observations(uint256)` and selector `[37, 44, 9, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `observe` function with signature `observe(uint32[])` and selector `[136, 59, 219, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `positions` function with signature `positions(bytes32)` and selector `[81, 78, 164, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "positions", abi = "positions(bytes32)")]
    pub struct PositionsCall {
        pub key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `protocolFees` function with signature `protocolFees()` and selector `[26, 216, 176, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees()")]
    pub struct ProtocolFeesCall;
    #[doc = "Container type for all input parameters for the `setFeeProtocol` function with signature `setFeeProtocol(uint8,uint8)` and selector `[130, 6, 164, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    #[doc = "Container type for all input parameters for the `slot0` function with signature `slot0()` and selector `[56, 80, 199, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "slot0", abi = "slot0()")]
    pub struct Slot0Call;
    #[doc = "Container type for all input parameters for the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `[163, 136, 7, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "snapshotCumulativesInside", abi = "snapshotCumulativesInside(int24,int24)")]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `[18, 138, 203, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swap", abi = "swap(address,bool,int256,uint160,bytes)")]
    pub struct SwapCall {
        pub recipient: ethers::core::types::Address,
        pub zero_for_one: bool,
        pub amount_specified: I256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `tickBitmap` function with signature `tickBitmap(int16)` and selector `[83, 57, 194, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tickBitmap", abi = "tickBitmap(int16)")]
    pub struct TickBitmapCall {
        pub word_position: i16,
    }
    #[doc = "Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    #[doc = "Container type for all input parameters for the `ticks` function with signature `ticks(int24)` and selector `[243, 13, 186, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ticks", abi = "ticks(int24)")]
    pub struct TicksCall {
        pub tick: i32,
    }
    #[doc = "Container type for all input parameters for the `token0` function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1` function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolCalls {
        Burn(BurnCall),
        Collect(CollectCall),
        CollectProtocol(CollectProtocolCall),
        Factory(FactoryCall),
        Fee(FeeCall),
        FeeGrowthGlobal0X128(FeeGrowthGlobal0X128Call),
        FeeGrowthGlobal1X128(FeeGrowthGlobal1X128Call),
        Flash(FlashCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        Initialize(InitializeCall),
        Liquidity(LiquidityCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        Mint(MintCall),
        Observations(ObservationsCall),
        Observe(ObserveCall),
        Positions(PositionsCall),
        ProtocolFees(ProtocolFeesCall),
        SetFeeProtocol(SetFeeProtocolCall),
        Slot0(Slot0Call),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ethers::core::abi::AbiDecode for IUniswapV3PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolCalls::Burn(decoded))
            }
            if let Ok(decoded) =
                <CollectCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Collect(decoded))
            }
            if let Ok(decoded) =
                <CollectProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::CollectProtocol(decoded))
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Factory(decoded))
            }
            if let Ok(decoded) = <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolCalls::Fee(decoded))
            }
            if let Ok(decoded) =
                <FeeGrowthGlobal0X128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::FeeGrowthGlobal0X128(decoded))
            }
            if let Ok(decoded) =
                <FeeGrowthGlobal1X128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::FeeGrowthGlobal1X128(decoded))
            }
            if let Ok(decoded) = <FlashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Flash(decoded))
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV3PoolCalls::IncreaseObservationCardinalityNext(decoded))
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Initialize(decoded))
            }
            if let Ok(decoded) =
                <LiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Liquidity(decoded))
            }
            if let Ok(decoded) =
                <MaxLiquidityPerTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::MaxLiquidityPerTick(decoded))
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolCalls::Mint(decoded))
            }
            if let Ok(decoded) =
                <ObservationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Observations(decoded))
            }
            if let Ok(decoded) =
                <ObserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Observe(decoded))
            }
            if let Ok(decoded) =
                <PositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Positions(decoded))
            }
            if let Ok(decoded) =
                <ProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::ProtocolFees(decoded))
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::SetFeeProtocol(decoded))
            }
            if let Ok(decoded) = <Slot0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Slot0(decoded))
            }
            if let Ok(decoded) =
                <SnapshotCumulativesInsideCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV3PoolCalls::SnapshotCumulativesInside(decoded))
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolCalls::Swap(decoded))
            }
            if let Ok(decoded) =
                <TickBitmapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::TickBitmap(decoded))
            }
            if let Ok(decoded) =
                <TickSpacingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::TickSpacing(decoded))
            }
            if let Ok(decoded) = <TicksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Ticks(decoded))
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Token0(decoded))
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolCalls::Token1(decoded))
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IUniswapV3PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolCalls::Burn(element) => element.encode(),
                IUniswapV3PoolCalls::Collect(element) => element.encode(),
                IUniswapV3PoolCalls::CollectProtocol(element) => element.encode(),
                IUniswapV3PoolCalls::Factory(element) => element.encode(),
                IUniswapV3PoolCalls::Fee(element) => element.encode(),
                IUniswapV3PoolCalls::FeeGrowthGlobal0X128(element) => element.encode(),
                IUniswapV3PoolCalls::FeeGrowthGlobal1X128(element) => element.encode(),
                IUniswapV3PoolCalls::Flash(element) => element.encode(),
                IUniswapV3PoolCalls::IncreaseObservationCardinalityNext(element) => {
                    element.encode()
                }
                IUniswapV3PoolCalls::Initialize(element) => element.encode(),
                IUniswapV3PoolCalls::Liquidity(element) => element.encode(),
                IUniswapV3PoolCalls::MaxLiquidityPerTick(element) => element.encode(),
                IUniswapV3PoolCalls::Mint(element) => element.encode(),
                IUniswapV3PoolCalls::Observations(element) => element.encode(),
                IUniswapV3PoolCalls::Observe(element) => element.encode(),
                IUniswapV3PoolCalls::Positions(element) => element.encode(),
                IUniswapV3PoolCalls::ProtocolFees(element) => element.encode(),
                IUniswapV3PoolCalls::SetFeeProtocol(element) => element.encode(),
                IUniswapV3PoolCalls::Slot0(element) => element.encode(),
                IUniswapV3PoolCalls::SnapshotCumulativesInside(element) => element.encode(),
                IUniswapV3PoolCalls::Swap(element) => element.encode(),
                IUniswapV3PoolCalls::TickBitmap(element) => element.encode(),
                IUniswapV3PoolCalls::TickSpacing(element) => element.encode(),
                IUniswapV3PoolCalls::Ticks(element) => element.encode(),
                IUniswapV3PoolCalls::Token0(element) => element.encode(),
                IUniswapV3PoolCalls::Token1(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolCalls::Burn(element) => element.fmt(f),
                IUniswapV3PoolCalls::Collect(element) => element.fmt(f),
                IUniswapV3PoolCalls::CollectProtocol(element) => element.fmt(f),
                IUniswapV3PoolCalls::Factory(element) => element.fmt(f),
                IUniswapV3PoolCalls::Fee(element) => element.fmt(f),
                IUniswapV3PoolCalls::FeeGrowthGlobal0X128(element) => element.fmt(f),
                IUniswapV3PoolCalls::FeeGrowthGlobal1X128(element) => element.fmt(f),
                IUniswapV3PoolCalls::Flash(element) => element.fmt(f),
                IUniswapV3PoolCalls::IncreaseObservationCardinalityNext(element) => element.fmt(f),
                IUniswapV3PoolCalls::Initialize(element) => element.fmt(f),
                IUniswapV3PoolCalls::Liquidity(element) => element.fmt(f),
                IUniswapV3PoolCalls::MaxLiquidityPerTick(element) => element.fmt(f),
                IUniswapV3PoolCalls::Mint(element) => element.fmt(f),
                IUniswapV3PoolCalls::Observations(element) => element.fmt(f),
                IUniswapV3PoolCalls::Observe(element) => element.fmt(f),
                IUniswapV3PoolCalls::Positions(element) => element.fmt(f),
                IUniswapV3PoolCalls::ProtocolFees(element) => element.fmt(f),
                IUniswapV3PoolCalls::SetFeeProtocol(element) => element.fmt(f),
                IUniswapV3PoolCalls::Slot0(element) => element.fmt(f),
                IUniswapV3PoolCalls::SnapshotCumulativesInside(element) => element.fmt(f),
                IUniswapV3PoolCalls::Swap(element) => element.fmt(f),
                IUniswapV3PoolCalls::TickBitmap(element) => element.fmt(f),
                IUniswapV3PoolCalls::TickSpacing(element) => element.fmt(f),
                IUniswapV3PoolCalls::Ticks(element) => element.fmt(f),
                IUniswapV3PoolCalls::Token0(element) => element.fmt(f),
                IUniswapV3PoolCalls::Token1(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCall> for IUniswapV3PoolCalls {
        fn from(var: BurnCall) -> Self {
            IUniswapV3PoolCalls::Burn(var)
        }
    }
    impl ::std::convert::From<CollectCall> for IUniswapV3PoolCalls {
        fn from(var: CollectCall) -> Self {
            IUniswapV3PoolCalls::Collect(var)
        }
    }
    impl ::std::convert::From<CollectProtocolCall> for IUniswapV3PoolCalls {
        fn from(var: CollectProtocolCall) -> Self {
            IUniswapV3PoolCalls::CollectProtocol(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IUniswapV3PoolCalls {
        fn from(var: FactoryCall) -> Self {
            IUniswapV3PoolCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeCall> for IUniswapV3PoolCalls {
        fn from(var: FeeCall) -> Self {
            IUniswapV3PoolCalls::Fee(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobal0X128Call> for IUniswapV3PoolCalls {
        fn from(var: FeeGrowthGlobal0X128Call) -> Self {
            IUniswapV3PoolCalls::FeeGrowthGlobal0X128(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobal1X128Call> for IUniswapV3PoolCalls {
        fn from(var: FeeGrowthGlobal1X128Call) -> Self {
            IUniswapV3PoolCalls::FeeGrowthGlobal1X128(var)
        }
    }
    impl ::std::convert::From<FlashCall> for IUniswapV3PoolCalls {
        fn from(var: FlashCall) -> Self {
            IUniswapV3PoolCalls::Flash(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for IUniswapV3PoolCalls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            IUniswapV3PoolCalls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IUniswapV3PoolCalls {
        fn from(var: InitializeCall) -> Self {
            IUniswapV3PoolCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LiquidityCall> for IUniswapV3PoolCalls {
        fn from(var: LiquidityCall) -> Self {
            IUniswapV3PoolCalls::Liquidity(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall> for IUniswapV3PoolCalls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            IUniswapV3PoolCalls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<MintCall> for IUniswapV3PoolCalls {
        fn from(var: MintCall) -> Self {
            IUniswapV3PoolCalls::Mint(var)
        }
    }
    impl ::std::convert::From<ObservationsCall> for IUniswapV3PoolCalls {
        fn from(var: ObservationsCall) -> Self {
            IUniswapV3PoolCalls::Observations(var)
        }
    }
    impl ::std::convert::From<ObserveCall> for IUniswapV3PoolCalls {
        fn from(var: ObserveCall) -> Self {
            IUniswapV3PoolCalls::Observe(var)
        }
    }
    impl ::std::convert::From<PositionsCall> for IUniswapV3PoolCalls {
        fn from(var: PositionsCall) -> Self {
            IUniswapV3PoolCalls::Positions(var)
        }
    }
    impl ::std::convert::From<ProtocolFeesCall> for IUniswapV3PoolCalls {
        fn from(var: ProtocolFeesCall) -> Self {
            IUniswapV3PoolCalls::ProtocolFees(var)
        }
    }
    impl ::std::convert::From<SetFeeProtocolCall> for IUniswapV3PoolCalls {
        fn from(var: SetFeeProtocolCall) -> Self {
            IUniswapV3PoolCalls::SetFeeProtocol(var)
        }
    }
    impl ::std::convert::From<Slot0Call> for IUniswapV3PoolCalls {
        fn from(var: Slot0Call) -> Self {
            IUniswapV3PoolCalls::Slot0(var)
        }
    }
    impl ::std::convert::From<SnapshotCumulativesInsideCall> for IUniswapV3PoolCalls {
        fn from(var: SnapshotCumulativesInsideCall) -> Self {
            IUniswapV3PoolCalls::SnapshotCumulativesInside(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IUniswapV3PoolCalls {
        fn from(var: SwapCall) -> Self {
            IUniswapV3PoolCalls::Swap(var)
        }
    }
    impl ::std::convert::From<TickBitmapCall> for IUniswapV3PoolCalls {
        fn from(var: TickBitmapCall) -> Self {
            IUniswapV3PoolCalls::TickBitmap(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for IUniswapV3PoolCalls {
        fn from(var: TickSpacingCall) -> Self {
            IUniswapV3PoolCalls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<TicksCall> for IUniswapV3PoolCalls {
        fn from(var: TicksCall) -> Self {
            IUniswapV3PoolCalls::Ticks(var)
        }
    }
    impl ::std::convert::From<Token0Call> for IUniswapV3PoolCalls {
        fn from(var: Token0Call) -> Self {
            IUniswapV3PoolCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for IUniswapV3PoolCalls {
        fn from(var: Token1Call) -> Self {
            IUniswapV3PoolCalls::Token1(var)
        }
    }
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(int24,int24,uint128)` and selector `[163, 65, 35, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BurnReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `[79, 30, 179, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CollectReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[doc = "Container type for all return fields from the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `[133, 182, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CollectProtocolReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[doc = "Container type for all return fields from the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FactoryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `fee` function with signature `fee()` and selector `[221, 202, 63, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeReturn(pub u32);
    #[doc = "Container type for all return fields from the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `[243, 5, 131, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeGrowthGlobal0X128Return(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `[70, 20, 19, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeGrowthGlobal1X128Return(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `liquidity` function with signature `liquidity()` and selector `[26, 104, 101, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidityReturn(pub u128);
    #[doc = "Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxLiquidityPerTickReturn(pub u128);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `[60, 138, 125, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `observations` function with signature `observations(uint256)` and selector `[37, 44, 9, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ObservationsReturn {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub seconds_per_liquidity_cumulative_x128: ethers::core::types::U256,
        pub initialized: bool,
    }
    #[doc = "Container type for all return fields from the `observe` function with signature `observe(uint32[])` and selector `[136, 59, 219, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `positions` function with signature `positions(bytes32)` and selector `[81, 78, 164, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PositionsReturn {
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ethers::core::types::U256,
        pub tokens_owed_0: u128,
        pub tokens_owed_1: u128,
    }
    #[doc = "Container type for all return fields from the `protocolFees` function with signature `protocolFees()` and selector `[26, 216, 176, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProtocolFeesReturn {
        pub token_0: u128,
        pub token_1: u128,
    }
    #[doc = "Container type for all return fields from the `slot0` function with signature `slot0()` and selector `[56, 80, 199, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Slot0Return {
        pub sqrt_price_x96: ethers::core::types::U256,
        pub tick: i32,
        pub observation_index: u16,
        pub observation_cardinality: u16,
        pub observation_cardinality_next: u16,
        pub fee_protocol: u8,
        pub unlocked: bool,
    }
    #[doc = "Container type for all return fields from the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `[163, 136, 7, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SnapshotCumulativesInsideReturn {
        pub tick_cumulative_inside: i64,
        pub seconds_per_liquidity_inside_x128: ethers::core::types::U256,
        pub seconds_inside: u32,
    }
    #[doc = "Container type for all return fields from the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `[18, 138, 203, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapReturn {
        pub amount_0: I256,
        pub amount_1: I256,
    }
    #[doc = "Container type for all return fields from the `tickBitmap` function with signature `tickBitmap(int16)` and selector `[83, 57, 194, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TickBitmapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TickSpacingReturn(pub i32);
    #[doc = "Container type for all return fields from the `ticks` function with signature `ticks(int24)` and selector `[243, 13, 186, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TicksReturn {
        pub liquidity_gross: u128,
        pub liquidity_net: i128,
        pub fee_growth_outside_0x128: ethers::core::types::U256,
        pub fee_growth_outside_1x128: ethers::core::types::U256,
        pub tick_cumulative_outside: i64,
        pub seconds_per_liquidity_outside_x128: ethers::core::types::U256,
        pub seconds_outside: u32,
        pub initialized: bool,
    }
    #[doc = "Container type for all return fields from the `token0` function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Token0Return(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `token1` function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Token1Return(pub ethers::core::types::Address);
}
