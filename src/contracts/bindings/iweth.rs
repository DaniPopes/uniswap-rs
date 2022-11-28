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
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n        \"constant\": true,\n        \"inputs\": [],\n        \"name\": \"name\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"string\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": false,\n        \"inputs\": [\n            { \"name\": \"guy\", \"type\": \"address\" },\n            { \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n        \"payable\": false,\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": true,\n        \"inputs\": [],\n        \"name\": \"totalSupply\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"uint256\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": false,\n        \"inputs\": [\n            { \"name\": \"src\", \"type\": \"address\" },\n            { \"name\": \"dst\", \"type\": \"address\" },\n            { \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"transferFrom\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n        \"payable\": false,\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": false,\n        \"inputs\": [{ \"name\": \"wad\", \"type\": \"uint256\" }],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"payable\": false,\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": true,\n        \"inputs\": [],\n        \"name\": \"decimals\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"uint8\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": true,\n        \"inputs\": [{ \"name\": \"\", \"type\": \"address\" }],\n        \"name\": \"balanceOf\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"uint256\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": true,\n        \"inputs\": [],\n        \"name\": \"symbol\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"string\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": false,\n        \"inputs\": [\n            { \"name\": \"dst\", \"type\": \"address\" },\n            { \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n        \"payable\": false,\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": false,\n        \"inputs\": [],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"payable\": true,\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"constant\": true,\n        \"inputs\": [\n            { \"name\": \"\", \"type\": \"address\" },\n            { \"name\": \"\", \"type\": \"address\" }\n        ],\n        \"name\": \"allowance\",\n        \"outputs\": [{ \"name\": \"\", \"type\": \"uint256\" }],\n        \"payable\": false,\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    { \"payable\": true, \"stateMutability\": \"payable\", \"type\": \"fallback\" },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            { \"indexed\": true, \"name\": \"src\", \"type\": \"address\" },\n            { \"indexed\": true, \"name\": \"guy\", \"type\": \"address\" },\n            { \"indexed\": false, \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"Approval\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            { \"indexed\": true, \"name\": \"src\", \"type\": \"address\" },\n            { \"indexed\": true, \"name\": \"dst\", \"type\": \"address\" },\n            { \"indexed\": false, \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"Transfer\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            { \"indexed\": true, \"name\": \"dst\", \"type\": \"address\" },\n            { \"indexed\": false, \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"Deposit\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            { \"indexed\": true, \"name\": \"src\", \"type\": \"address\" },\n            { \"indexed\": false, \"name\": \"wad\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"Withdrawal\",\n        \"type\": \"event\"\n    }\n]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IWETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
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
    impl<M> std::fmt::Debug for IWETH<M> {
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
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            guy: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (guy, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xd0e30db0) function"]
        pub fn deposit(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            dst: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], wad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdrawal` event"]
        pub fn withdrawal_filter(&self) -> ethers::contract::builders::Event<M, WithdrawalFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IWETHEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IWETH<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub src: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub guy: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub src: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,uint256)")]
    pub struct WithdrawalFilter {
        #[ethevent(indexed)]
        pub src: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWETHEvents {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        TransferFilter(TransferFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ethers::contract::EthLogDecode for IWETHEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IWETHEvents::ApprovalFilter(decoded))
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(IWETHEvents::DepositFilter(decoded))
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IWETHEvents::TransferFilter(decoded))
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(IWETHEvents::WithdrawalFilter(decoded))
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IWETHEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWETHEvents::ApprovalFilter(element) => element.fmt(f),
                IWETHEvents::DepositFilter(element) => element.fmt(f),
                IWETHEvents::TransferFilter(element) => element.fmt(f),
                IWETHEvents::WithdrawalFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(pub ethers::core::types::Address, pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub guy: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
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
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
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
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
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
    pub struct WithdrawCall {
        pub wad: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWETHCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        Name(NameCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IWETHCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Allowance(decoded))
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Approve(decoded))
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::BalanceOf(decoded))
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Decimals(decoded))
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Deposit(decoded))
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IWETHCalls::Name(decoded))
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Symbol(decoded))
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::TotalSupply(decoded))
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::Transfer(decoded))
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHCalls::TransferFrom(decoded))
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
                IWETHCalls::Allowance(element) => element.encode(),
                IWETHCalls::Approve(element) => element.encode(),
                IWETHCalls::BalanceOf(element) => element.encode(),
                IWETHCalls::Decimals(element) => element.encode(),
                IWETHCalls::Deposit(element) => element.encode(),
                IWETHCalls::Name(element) => element.encode(),
                IWETHCalls::Symbol(element) => element.encode(),
                IWETHCalls::TotalSupply(element) => element.encode(),
                IWETHCalls::Transfer(element) => element.encode(),
                IWETHCalls::TransferFrom(element) => element.encode(),
                IWETHCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IWETHCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWETHCalls::Allowance(element) => element.fmt(f),
                IWETHCalls::Approve(element) => element.fmt(f),
                IWETHCalls::BalanceOf(element) => element.fmt(f),
                IWETHCalls::Decimals(element) => element.fmt(f),
                IWETHCalls::Deposit(element) => element.fmt(f),
                IWETHCalls::Name(element) => element.fmt(f),
                IWETHCalls::Symbol(element) => element.fmt(f),
                IWETHCalls::TotalSupply(element) => element.fmt(f),
                IWETHCalls::Transfer(element) => element.fmt(f),
                IWETHCalls::TransferFrom(element) => element.fmt(f),
                IWETHCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllowanceCall> for IWETHCalls {
        fn from(var: AllowanceCall) -> Self {
            IWETHCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for IWETHCalls {
        fn from(var: ApproveCall) -> Self {
            IWETHCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IWETHCalls {
        fn from(var: BalanceOfCall) -> Self {
            IWETHCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for IWETHCalls {
        fn from(var: DecimalsCall) -> Self {
            IWETHCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DepositCall> for IWETHCalls {
        fn from(var: DepositCall) -> Self {
            IWETHCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<NameCall> for IWETHCalls {
        fn from(var: NameCall) -> Self {
            IWETHCalls::Name(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for IWETHCalls {
        fn from(var: SymbolCall) -> Self {
            IWETHCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IWETHCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IWETHCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for IWETHCalls {
        fn from(var: TransferCall) -> Self {
            IWETHCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for IWETHCalls {
        fn from(var: TransferFromCall) -> Self {
            IWETHCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IWETHCalls {
        fn from(var: WithdrawCall) -> Self {
            IWETHCalls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
