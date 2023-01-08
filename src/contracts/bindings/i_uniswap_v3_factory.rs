pub use i_uniswap_v3_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_uniswap_v3_factory {
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
    #[doc = "IUniswapV3Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"indexed\":true,\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"}],\"name\":\"FeeAmountEnabled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"indexed\":false,\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"name\":\"PoolCreated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"}],\"name\":\"createPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"}],\"name\":\"enableFeeAmount\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"}],\"name\":\"feeAmountTickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"}],\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IUNISWAPV3FACTORY_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            ethers_core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IUniswapV3Factory<M>(ethers_contract::Contract<M>);
    impl<M> Clone for IUniswapV3Factory<M> {
        fn clone(&self) -> Self {
            IUniswapV3Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV3Factory<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV3Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3Factory)).field(&self.address()).finish()
        }
    }
    impl<M: ethers_providers::Middleware> IUniswapV3Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers_contract::Contract::new(address.into(), IUNISWAPV3FACTORY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `createPool` (0xa1671295) function"]
        pub fn create_pool(
            &self,
            token_a: ethers_core::types::Address,
            token_b: ethers_core::types::Address,
            fee: u32,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([161, 103, 18, 149], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableFeeAmount` (0x8a7c195f) function"]
        pub fn enable_fee_amount(
            &self,
            fee: u32,
            tick_spacing: i32,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 124, 25, 95], (fee, tick_spacing))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeAmountTickSpacing` (0x22afcccb) function"]
        pub fn fee_amount_tick_spacing(
            &self,
            fee: u32,
        ) -> ethers_contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([34, 175, 204, 203], fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPool` (0x1698ee82) function"]
        pub fn get_pool(
            &self,
            token_a: ethers_core::types::Address,
            token_b: ethers_core::types::Address,
            fee: u32,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([22, 152, 238, 130], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FeeAmountEnabled` event"]
        pub fn fee_amount_enabled_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, FeeAmountEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerChanged` event"]
        pub fn owner_changed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, OwnerChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolCreated` event"]
        pub fn pool_created_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PoolCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, IUniswapV3FactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers_providers::Middleware> From<ethers_contract::Contract<M>> for IUniswapV3Factory<M> {
        fn from(contract: ethers_contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthEvent,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "FeeAmountEnabled", abi = "FeeAmountEnabled(uint24,int24)")]
    pub struct FeeAmountEnabledFilter {
        #[ethevent(indexed)]
        pub fee: u32,
        #[ethevent(indexed)]
        pub tick_spacing: i32,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthEvent,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        #[ethevent(indexed)]
        pub old_owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers_core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthEvent,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "PoolCreated", abi = "PoolCreated(address,address,uint24,int24,address)")]
    pub struct PoolCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub fee: u32,
        pub tick_spacing: i32,
        pub pool: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum IUniswapV3FactoryEvents {
        FeeAmountEnabledFilter(FeeAmountEnabledFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        PoolCreatedFilter(PoolCreatedFilter),
    }
    impl ethers_contract::EthLogDecode for IUniswapV3FactoryEvents {
        fn decode_log(
            log: &ethers_core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeeAmountEnabledFilter::decode_log(log) {
                return Ok(IUniswapV3FactoryEvents::FeeAmountEnabledFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(IUniswapV3FactoryEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolCreatedFilter::decode_log(log) {
                return Ok(IUniswapV3FactoryEvents::PoolCreatedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IUniswapV3FactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3FactoryEvents::FeeAmountEnabledFilter(element) => element.fmt(f),
                IUniswapV3FactoryEvents::OwnerChangedFilter(element) => element.fmt(f),
                IUniswapV3FactoryEvents::PoolCreatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `createPool` function with signature `createPool(address,address,uint24)` and selector `[161, 103, 18, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createPool", abi = "createPool(address,address,uint24)")]
    pub struct CreatePoolCall {
        pub token_a: ethers_core::types::Address,
        pub token_b: ethers_core::types::Address,
        pub fee: u32,
    }
    #[doc = "Container type for all input parameters for the `enableFeeAmount` function with signature `enableFeeAmount(uint24,int24)` and selector `[138, 124, 25, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "enableFeeAmount", abi = "enableFeeAmount(uint24,int24)")]
    pub struct EnableFeeAmountCall {
        pub fee: u32,
        pub tick_spacing: i32,
    }
    #[doc = "Container type for all input parameters for the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `[34, 175, 204, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeAmountTickSpacing", abi = "feeAmountTickSpacing(uint24)")]
    pub struct FeeAmountTickSpacingCall {
        pub fee: u32,
    }
    #[doc = "Container type for all input parameters for the `getPool` function with signature `getPool(address,address,uint24)` and selector `[22, 152, 238, 130]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPool", abi = "getPool(address,address,uint24)")]
    pub struct GetPoolCall {
        pub token_a: ethers_core::types::Address,
        pub token_b: ethers_core::types::Address,
        pub fee: u32,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum IUniswapV3FactoryCalls {
        CreatePool(CreatePoolCall),
        EnableFeeAmount(EnableFeeAmountCall),
        FeeAmountTickSpacing(FeeAmountTickSpacingCall),
        GetPool(GetPoolCall),
        Owner(OwnerCall),
        SetOwner(SetOwnerCall),
    }
    impl ethers_core::abi::AbiDecode for IUniswapV3FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers_core::abi::AbiError> {
            if let Ok(decoded) =
                <CreatePoolCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3FactoryCalls::CreatePool(decoded));
            }
            if let Ok(decoded) =
                <EnableFeeAmountCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3FactoryCalls::EnableFeeAmount(decoded));
            }
            if let Ok(decoded) =
                <FeeAmountTickSpacingCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3FactoryCalls::FeeAmountTickSpacing(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3FactoryCalls::GetPool(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3FactoryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3FactoryCalls::SetOwner(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for IUniswapV3FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3FactoryCalls::CreatePool(element) => element.encode(),
                IUniswapV3FactoryCalls::EnableFeeAmount(element) => element.encode(),
                IUniswapV3FactoryCalls::FeeAmountTickSpacing(element) => element.encode(),
                IUniswapV3FactoryCalls::GetPool(element) => element.encode(),
                IUniswapV3FactoryCalls::Owner(element) => element.encode(),
                IUniswapV3FactoryCalls::SetOwner(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3FactoryCalls::CreatePool(element) => element.fmt(f),
                IUniswapV3FactoryCalls::EnableFeeAmount(element) => element.fmt(f),
                IUniswapV3FactoryCalls::FeeAmountTickSpacing(element) => element.fmt(f),
                IUniswapV3FactoryCalls::GetPool(element) => element.fmt(f),
                IUniswapV3FactoryCalls::Owner(element) => element.fmt(f),
                IUniswapV3FactoryCalls::SetOwner(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreatePoolCall> for IUniswapV3FactoryCalls {
        fn from(var: CreatePoolCall) -> Self {
            IUniswapV3FactoryCalls::CreatePool(var)
        }
    }
    impl ::std::convert::From<EnableFeeAmountCall> for IUniswapV3FactoryCalls {
        fn from(var: EnableFeeAmountCall) -> Self {
            IUniswapV3FactoryCalls::EnableFeeAmount(var)
        }
    }
    impl ::std::convert::From<FeeAmountTickSpacingCall> for IUniswapV3FactoryCalls {
        fn from(var: FeeAmountTickSpacingCall) -> Self {
            IUniswapV3FactoryCalls::FeeAmountTickSpacing(var)
        }
    }
    impl ::std::convert::From<GetPoolCall> for IUniswapV3FactoryCalls {
        fn from(var: GetPoolCall) -> Self {
            IUniswapV3FactoryCalls::GetPool(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IUniswapV3FactoryCalls {
        fn from(var: OwnerCall) -> Self {
            IUniswapV3FactoryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for IUniswapV3FactoryCalls {
        fn from(var: SetOwnerCall) -> Self {
            IUniswapV3FactoryCalls::SetOwner(var)
        }
    }
    #[doc = "Container type for all return fields from the `createPool` function with signature `createPool(address,address,uint24)` and selector `[161, 103, 18, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreatePoolReturn {
        pub pool: ethers_core::types::Address,
    }
    #[doc = "Container type for all return fields from the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `[34, 175, 204, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeAmountTickSpacingReturn(pub i32);
    #[doc = "Container type for all return fields from the `getPool` function with signature `getPool(address,address,uint24)` and selector `[22, 152, 238, 130]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPoolReturn {
        pub pool: ethers_core::types::Address,
    }
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers_core::types::Address);
}
