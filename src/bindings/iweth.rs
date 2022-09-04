pub use iweth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod iweth {
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
    #[doc = "IWETH was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IWETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n    {\n        \"inputs\": [],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    pub struct IWETH<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IWETH<M> {
        fn clone(&self) -> Self {
            IWETH(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IWETH<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IWETH<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IWETH)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> IWETH<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IWETH_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `deposit` (0xd0e30db0) function"]
        pub fn deposit(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IWETH<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `[208, 227, 13, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWETHCalls {
        Deposit(DepositCall),
        Transfer(TransferCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IWETHCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Deposit(decoded))
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Transfer(decoded))
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Withdraw(decoded))
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IWETHCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IWETHCalls::Deposit(element) => element.encode(),
                IWETHCalls::Transfer(element) => element.encode(),
                IWETHCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IWETHCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWETHCalls::Deposit(element) => element.fmt(f),
                IWETHCalls::Transfer(element) => element.fmt(f),
                IWETHCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DepositCall> for IWETHCalls {
        fn from(var: DepositCall) -> Self {
            IWETHCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<TransferCall> for IWETHCalls {
        fn from(var: TransferCall) -> Self {
            IWETHCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IWETHCalls {
        fn from(var: WithdrawCall) -> Self {
            IWETHCalls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferReturn(pub bool);
}
