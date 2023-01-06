pub use i_swap_router::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_swap_router {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers_contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers_core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers_providers::Middleware;
    #[doc = "ISwapRouter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"internalType\":\"struct ISwapRouter.ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\"}],\"internalType\":\"struct ISwapRouter.ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\"}],\"internalType\":\"struct ISwapRouter.ExactOutputParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\"},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\"}],\"internalType\":\"struct ISwapRouter.ExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\"},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3SwapCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ISWAPROUTER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            ethers_core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ISwapRouter<M>(ethers_contract::Contract<M>);
    impl<M> Clone for ISwapRouter<M> {
        fn clone(&self) -> Self {
            ISwapRouter(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ISwapRouter<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ISwapRouter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ISwapRouter)).field(&self.address()).finish()
        }
    }
    impl<M: ethers_providers::Middleware> ISwapRouter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers_contract::Contract::new(address.into(), ISWAPROUTER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `exactInput` (0xc04b8d59) function"]
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInputSingle` (0x414bf389) function"]
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutput` (0xf28c0498) function"]
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutputSingle` (0xdb3e2198) function"]
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function"]
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: I256,
            amount_1_delta: I256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers_providers::Middleware> From<ethers_contract::Contract<M>> for ISwapRouter<M> {
        fn from(contract: ethers_contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `[192, 75, 141, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exactInput", abi = "exactInput((bytes,address,uint256,uint256,uint256))")]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    #[doc = "Container type for all input parameters for the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[65, 75, 243, 137]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `[242, 140, 4, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exactOutput", abi = "exactOutput((bytes,address,uint256,uint256,uint256))")]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[219, 62, 33, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `[250, 70, 30, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "uniswapV3SwapCallback", abi = "uniswapV3SwapCallback(int256,int256,bytes)")]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: I256,
        pub amount_1_delta: I256,
        pub data: ethers_core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum ISwapRouterCalls {
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ethers_core::abi::AbiDecode for ISwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers_core::abi::AbiError> {
            if let Ok(decoded) =
                <ExactInputCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapRouterCalls::ExactInput(decoded))
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapRouterCalls::ExactInputSingle(decoded))
            }
            if let Ok(decoded) =
                <ExactOutputCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapRouterCalls::ExactOutput(decoded))
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapRouterCalls::ExactOutputSingle(decoded))
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapRouterCalls::UniswapV3SwapCallback(decoded))
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for ISwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ISwapRouterCalls::ExactInput(element) => element.encode(),
                ISwapRouterCalls::ExactInputSingle(element) => element.encode(),
                ISwapRouterCalls::ExactOutput(element) => element.encode(),
                ISwapRouterCalls::ExactOutputSingle(element) => element.encode(),
                ISwapRouterCalls::UniswapV3SwapCallback(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ISwapRouterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ISwapRouterCalls::ExactInput(element) => element.fmt(f),
                ISwapRouterCalls::ExactInputSingle(element) => element.fmt(f),
                ISwapRouterCalls::ExactOutput(element) => element.fmt(f),
                ISwapRouterCalls::ExactOutputSingle(element) => element.fmt(f),
                ISwapRouterCalls::UniswapV3SwapCallback(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExactInputCall> for ISwapRouterCalls {
        fn from(var: ExactInputCall) -> Self {
            ISwapRouterCalls::ExactInput(var)
        }
    }
    impl ::std::convert::From<ExactInputSingleCall> for ISwapRouterCalls {
        fn from(var: ExactInputSingleCall) -> Self {
            ISwapRouterCalls::ExactInputSingle(var)
        }
    }
    impl ::std::convert::From<ExactOutputCall> for ISwapRouterCalls {
        fn from(var: ExactOutputCall) -> Self {
            ISwapRouterCalls::ExactOutput(var)
        }
    }
    impl ::std::convert::From<ExactOutputSingleCall> for ISwapRouterCalls {
        fn from(var: ExactOutputSingleCall) -> Self {
            ISwapRouterCalls::ExactOutputSingle(var)
        }
    }
    impl ::std::convert::From<UniswapV3SwapCallbackCall> for ISwapRouterCalls {
        fn from(var: UniswapV3SwapCallbackCall) -> Self {
            ISwapRouterCalls::UniswapV3SwapCallback(var)
        }
    }
    #[doc = "Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `[192, 75, 141, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExactInputReturn {
        pub amount_out: ethers_core::types::U256,
    }
    #[doc = "Container type for all return fields from the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[65, 75, 243, 137]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExactInputSingleReturn {
        pub amount_out: ethers_core::types::U256,
    }
    #[doc = "Container type for all return fields from the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `[242, 140, 4, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExactOutputReturn {
        pub amount_in: ethers_core::types::U256,
    }
    #[doc = "Container type for all return fields from the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[219, 62, 33, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExactOutputSingleReturn {
        pub amount_in: ethers_core::types::U256,
    }
    #[doc = "`ExactInputParams(bytes,address,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct ExactInputParams {
        pub path: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub deadline: ethers_core::types::U256,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_minimum: ethers_core::types::U256,
    }
    #[doc = "`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct ExactInputSingleParams {
        pub token_in: ethers_core::types::Address,
        pub token_out: ethers_core::types::Address,
        pub fee: u32,
        pub recipient: ethers_core::types::Address,
        pub deadline: ethers_core::types::U256,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_minimum: ethers_core::types::U256,
        pub sqrt_price_limit_x96: ethers_core::types::U256,
    }
    #[doc = "`ExactOutputParams(bytes,address,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct ExactOutputParams {
        pub path: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub deadline: ethers_core::types::U256,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_maximum: ethers_core::types::U256,
    }
    #[doc = "`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct ExactOutputSingleParams {
        pub token_in: ethers_core::types::Address,
        pub token_out: ethers_core::types::Address,
        pub fee: u32,
        pub recipient: ethers_core::types::Address,
        pub deadline: ethers_core::types::U256,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_maximum: ethers_core::types::U256,
        pub sqrt_price_limit_x96: ethers_core::types::U256,
    }
}
