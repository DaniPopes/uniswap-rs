pub use i_universal_router::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_universal_router {
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
    #[doc = "IUniversalRouter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"permit2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"weth9\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"seaport\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"nftxZap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"x2y2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"foundation\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"sudoswap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"nft20Zap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"cryptopunks\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRare\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"routerRewardsDistributor\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRareRewardsDistributor\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRareToken\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"v2Factory\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"v3Factory\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"pairInitCodeHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"poolInitCodeHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct RouterParameters\",\"name\":\"params\",\"type\":\"tuple\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"ContractLocked\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ETHNotAccepted\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commandIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\"}],\"name\":\"ExecutionFailed\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"FromAddressIsNotOwner\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InsufficientETH\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InsufficientToken\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidBips\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commandType\",\"type\":\"uint256\"}],\"name\":\"InvalidCommandType\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidOwnerERC1155\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidOwnerERC721\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidPath\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidReserves\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"LengthMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NoSlice\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SliceOutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SliceOverflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToAddressOutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToAddressOverflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToUint24OutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToUint24Overflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"TransactionDeadlinePassed\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnableToClaim\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnsafeCast\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2InvalidPath\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2TooLittleReceived\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2TooMuchRequested\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidAmountOut\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidCaller\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidSwap\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3TooLittleReceived\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3TooMuchRequested\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"RewardsSent\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"looksRareClaim\",\"type\":\"bytes\"}],\"name\":\"collectRewards\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"commands\",\"type\":\"bytes\"},{\"internalType\":\"bytes[]\",\"name\":\"inputs\",\"type\":\"bytes[]\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"commands\",\"type\":\"bytes\"},{\"internalType\":\"bytes[]\",\"name\":\"inputs\",\"type\":\"bytes[]\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC721Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\"},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3SwapCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IUNIVERSALROUTER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            ethers_core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IUniversalRouter<M>(ethers_contract::Contract<M>);
    impl<M> Clone for IUniversalRouter<M> {
        fn clone(&self) -> Self {
            IUniversalRouter(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniversalRouter<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniversalRouter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniversalRouter)).field(&self.address()).finish()
        }
    }
    impl<M: ethers_providers::Middleware> IUniversalRouter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers_contract::Contract::new(address.into(), IUNIVERSALROUTER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `collectRewards` (0x709a1cc2) function"]
        pub fn collect_rewards(
            &self,
            looks_rare_claim: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 154, 28, 194], looks_rare_claim)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x24856bc3) function"]
        pub fn execute(
            &self,
            commands: ethers_core::types::Bytes,
            inputs: ::std::vec::Vec<ethers_core::types::Bytes>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 133, 107, 195], (commands, inputs))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x3593564c) function"]
        pub fn execute_with_commands_and_inputs(
            &self,
            commands: ethers_core::types::Bytes,
            inputs: ::std::vec::Vec<ethers_core::types::Bytes>,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 147, 86, 76], (commands, inputs, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function"]
        pub fn on_erc1155_batch_received(
            &self,
            p0: ethers_core::types::Address,
            p1: ethers_core::types::Address,
            p2: ::std::vec::Vec<ethers_core::types::U256>,
            p3: ::std::vec::Vec<ethers_core::types::U256>,
            p4: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155Received` (0xf23a6e61) function"]
        pub fn on_erc1155_received(
            &self,
            p0: ethers_core::types::Address,
            p1: ethers_core::types::Address,
            p2: ethers_core::types::U256,
            p3: ethers_core::types::U256,
            p4: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC721Received` (0x150b7a02) function"]
        pub fn on_erc721_received(
            &self,
            p0: ethers_core::types::Address,
            p1: ethers_core::types::Address,
            p2: ethers_core::types::U256,
            p3: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        #[doc = "Gets the contract's `RewardsSent` event"]
        pub fn rewards_sent_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RewardsSentFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, RewardsSentFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers_providers::Middleware> From<ethers_contract::Contract<M>> for IUniversalRouter<M> {
        fn from(contract: ethers_contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `ContractLocked` with signature `ContractLocked()` and selector `[111, 95, 251, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ContractLocked", abi = "ContractLocked()")]
    pub struct ContractLocked;
    #[doc = "Custom Error type `ETHNotAccepted` with signature `ETHNotAccepted()` and selector `[18, 49, 174, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ETHNotAccepted", abi = "ETHNotAccepted()")]
    pub struct ETHNotAccepted;
    #[doc = "Custom Error type `ExecutionFailed` with signature `ExecutionFailed(uint256,bytes)` and selector `[44, 64, 41, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ExecutionFailed", abi = "ExecutionFailed(uint256,bytes)")]
    pub struct ExecutionFailed {
        pub command_index: ethers_core::types::U256,
        pub message: ethers_core::types::Bytes,
    }
    #[doc = "Custom Error type `FromAddressIsNotOwner` with signature `FromAddressIsNotOwner()` and selector `[231, 0, 40, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "FromAddressIsNotOwner", abi = "FromAddressIsNotOwner()")]
    pub struct FromAddressIsNotOwner;
    #[doc = "Custom Error type `InsufficientETH` with signature `InsufficientETH()` and selector `[106, 18, 241, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InsufficientETH", abi = "InsufficientETH()")]
    pub struct InsufficientETH;
    #[doc = "Custom Error type `InsufficientToken` with signature `InsufficientToken()` and selector `[103, 92, 174, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InsufficientToken", abi = "InsufficientToken()")]
    pub struct InsufficientToken;
    #[doc = "Custom Error type `InvalidBips` with signature `InvalidBips()` and selector `[222, 170, 1, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidBips", abi = "InvalidBips()")]
    pub struct InvalidBips;
    #[doc = "Custom Error type `InvalidCommandType` with signature `InvalidCommandType(uint256)` and selector `[215, 106, 30, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidCommandType", abi = "InvalidCommandType(uint256)")]
    pub struct InvalidCommandType {
        pub command_type: ethers_core::types::U256,
    }
    #[doc = "Custom Error type `InvalidOwnerERC1155` with signature `InvalidOwnerERC1155()` and selector `[72, 58, 105, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidOwnerERC1155", abi = "InvalidOwnerERC1155()")]
    pub struct InvalidOwnerERC1155;
    #[doc = "Custom Error type `InvalidOwnerERC721` with signature `InvalidOwnerERC721()` and selector `[125, 190, 126, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidOwnerERC721", abi = "InvalidOwnerERC721()")]
    pub struct InvalidOwnerERC721;
    #[doc = "Custom Error type `InvalidPath` with signature `InvalidPath()` and selector `[32, 219, 130, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidPath", abi = "InvalidPath()")]
    pub struct InvalidPath;
    #[doc = "Custom Error type `InvalidReserves` with signature `InvalidReserves()` and selector `[123, 156, 137, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidReserves", abi = "InvalidReserves()")]
    pub struct InvalidReserves;
    #[doc = "Custom Error type `LengthMismatch` with signature `LengthMismatch()` and selector `[255, 99, 58, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "LengthMismatch", abi = "LengthMismatch()")]
    pub struct LengthMismatch;
    #[doc = "Custom Error type `NoSlice` with signature `NoSlice()` and selector `[204, 148, 166, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "NoSlice", abi = "NoSlice()")]
    pub struct NoSlice;
    #[doc = "Custom Error type `SliceOutOfBounds` with signature `SliceOutOfBounds()` and selector `[59, 153, 181, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "SliceOutOfBounds", abi = "SliceOutOfBounds()")]
    pub struct SliceOutOfBounds;
    #[doc = "Custom Error type `SliceOverflow` with signature `SliceOverflow()` and selector `[71, 170, 240, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "SliceOverflow", abi = "SliceOverflow()")]
    pub struct SliceOverflow;
    #[doc = "Custom Error type `ToAddressOutOfBounds` with signature `ToAddressOutOfBounds()` and selector `[167, 138, 162, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ToAddressOutOfBounds", abi = "ToAddressOutOfBounds()")]
    pub struct ToAddressOutOfBounds;
    #[doc = "Custom Error type `ToAddressOverflow` with signature `ToAddressOverflow()` and selector `[119, 20, 110, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ToAddressOverflow", abi = "ToAddressOverflow()")]
    pub struct ToAddressOverflow;
    #[doc = "Custom Error type `ToUint24OutOfBounds` with signature `ToUint24OutOfBounds()` and selector `[217, 9, 106, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ToUint24OutOfBounds", abi = "ToUint24OutOfBounds()")]
    pub struct ToUint24OutOfBounds;
    #[doc = "Custom Error type `ToUint24Overflow` with signature `ToUint24Overflow()` and selector `[133, 88, 89, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "ToUint24Overflow", abi = "ToUint24Overflow()")]
    pub struct ToUint24Overflow;
    #[doc = "Custom Error type `TransactionDeadlinePassed` with signature `TransactionDeadlinePassed()` and selector `[91, 246, 249, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "TransactionDeadlinePassed", abi = "TransactionDeadlinePassed()")]
    pub struct TransactionDeadlinePassed;
    #[doc = "Custom Error type `UnableToClaim` with signature `UnableToClaim()` and selector `[125, 82, 153, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "UnableToClaim", abi = "UnableToClaim()")]
    pub struct UnableToClaim;
    #[doc = "Custom Error type `UnsafeCast` with signature `UnsafeCast()` and selector `[196, 189, 137, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "UnsafeCast", abi = "UnsafeCast()")]
    pub struct UnsafeCast;
    #[doc = "Custom Error type `V2InvalidPath` with signature `V2InvalidPath()` and selector `[174, 82, 173, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V2InvalidPath", abi = "V2InvalidPath()")]
    pub struct V2InvalidPath;
    #[doc = "Custom Error type `V2TooLittleReceived` with signature `V2TooLittleReceived()` and selector `[132, 158, 175, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V2TooLittleReceived", abi = "V2TooLittleReceived()")]
    pub struct V2TooLittleReceived;
    #[doc = "Custom Error type `V2TooMuchRequested` with signature `V2TooMuchRequested()` and selector `[138, 176, 188, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V2TooMuchRequested", abi = "V2TooMuchRequested()")]
    pub struct V2TooMuchRequested;
    #[doc = "Custom Error type `V3InvalidAmountOut` with signature `V3InvalidAmountOut()` and selector `[212, 224, 36, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V3InvalidAmountOut", abi = "V3InvalidAmountOut()")]
    pub struct V3InvalidAmountOut;
    #[doc = "Custom Error type `V3InvalidCaller` with signature `V3InvalidCaller()` and selector `[50, 177, 61, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V3InvalidCaller", abi = "V3InvalidCaller()")]
    pub struct V3InvalidCaller;
    #[doc = "Custom Error type `V3InvalidSwap` with signature `V3InvalidSwap()` and selector `[49, 108, 240, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V3InvalidSwap", abi = "V3InvalidSwap()")]
    pub struct V3InvalidSwap;
    #[doc = "Custom Error type `V3TooLittleReceived` with signature `V3TooLittleReceived()` and selector `[57, 211, 84, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V3TooLittleReceived", abi = "V3TooLittleReceived()")]
    pub struct V3TooLittleReceived;
    #[doc = "Custom Error type `V3TooMuchRequested` with signature `V3TooMuchRequested()` and selector `[115, 157, 190, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthError,
        ethers_contract :: EthDisplay,
    )]
    #[etherror(name = "V3TooMuchRequested", abi = "V3TooMuchRequested()")]
    pub struct V3TooMuchRequested;
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum IUniversalRouterErrors {
        ContractLocked(ContractLocked),
        ETHNotAccepted(ETHNotAccepted),
        ExecutionFailed(ExecutionFailed),
        FromAddressIsNotOwner(FromAddressIsNotOwner),
        InsufficientETH(InsufficientETH),
        InsufficientToken(InsufficientToken),
        InvalidBips(InvalidBips),
        InvalidCommandType(InvalidCommandType),
        InvalidOwnerERC1155(InvalidOwnerERC1155),
        InvalidOwnerERC721(InvalidOwnerERC721),
        InvalidPath(InvalidPath),
        InvalidReserves(InvalidReserves),
        LengthMismatch(LengthMismatch),
        NoSlice(NoSlice),
        SliceOutOfBounds(SliceOutOfBounds),
        SliceOverflow(SliceOverflow),
        ToAddressOutOfBounds(ToAddressOutOfBounds),
        ToAddressOverflow(ToAddressOverflow),
        ToUint24OutOfBounds(ToUint24OutOfBounds),
        ToUint24Overflow(ToUint24Overflow),
        TransactionDeadlinePassed(TransactionDeadlinePassed),
        UnableToClaim(UnableToClaim),
        UnsafeCast(UnsafeCast),
        V2InvalidPath(V2InvalidPath),
        V2TooLittleReceived(V2TooLittleReceived),
        V2TooMuchRequested(V2TooMuchRequested),
        V3InvalidAmountOut(V3InvalidAmountOut),
        V3InvalidCaller(V3InvalidCaller),
        V3InvalidSwap(V3InvalidSwap),
        V3TooLittleReceived(V3TooLittleReceived),
        V3TooMuchRequested(V3TooMuchRequested),
    }
    impl ethers_core::abi::AbiDecode for IUniversalRouterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers_core::abi::AbiError> {
            if let Ok(decoded) =
                <ContractLocked as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ContractLocked(decoded));
            }
            if let Ok(decoded) =
                <ETHNotAccepted as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ETHNotAccepted(decoded));
            }
            if let Ok(decoded) =
                <ExecutionFailed as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ExecutionFailed(decoded));
            }
            if let Ok(decoded) =
                <FromAddressIsNotOwner as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::FromAddressIsNotOwner(decoded));
            }
            if let Ok(decoded) =
                <InsufficientETH as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InsufficientETH(decoded));
            }
            if let Ok(decoded) =
                <InsufficientToken as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InsufficientToken(decoded));
            }
            if let Ok(decoded) = <InvalidBips as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidBips(decoded));
            }
            if let Ok(decoded) =
                <InvalidCommandType as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidCommandType(decoded));
            }
            if let Ok(decoded) =
                <InvalidOwnerERC1155 as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidOwnerERC1155(decoded));
            }
            if let Ok(decoded) =
                <InvalidOwnerERC721 as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidOwnerERC721(decoded));
            }
            if let Ok(decoded) = <InvalidPath as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidPath(decoded));
            }
            if let Ok(decoded) =
                <InvalidReserves as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::InvalidReserves(decoded));
            }
            if let Ok(decoded) =
                <LengthMismatch as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::LengthMismatch(decoded));
            }
            if let Ok(decoded) = <NoSlice as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterErrors::NoSlice(decoded));
            }
            if let Ok(decoded) =
                <SliceOutOfBounds as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::SliceOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <SliceOverflow as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::SliceOverflow(decoded));
            }
            if let Ok(decoded) =
                <ToAddressOutOfBounds as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ToAddressOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <ToAddressOverflow as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ToAddressOverflow(decoded));
            }
            if let Ok(decoded) =
                <ToUint24OutOfBounds as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ToUint24OutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <ToUint24Overflow as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::ToUint24Overflow(decoded));
            }
            if let Ok(decoded) =
                <TransactionDeadlinePassed as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::TransactionDeadlinePassed(decoded));
            }
            if let Ok(decoded) =
                <UnableToClaim as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::UnableToClaim(decoded));
            }
            if let Ok(decoded) = <UnsafeCast as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::UnsafeCast(decoded));
            }
            if let Ok(decoded) =
                <V2InvalidPath as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V2InvalidPath(decoded));
            }
            if let Ok(decoded) =
                <V2TooLittleReceived as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V2TooLittleReceived(decoded));
            }
            if let Ok(decoded) =
                <V2TooMuchRequested as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V2TooMuchRequested(decoded));
            }
            if let Ok(decoded) =
                <V3InvalidAmountOut as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V3InvalidAmountOut(decoded));
            }
            if let Ok(decoded) =
                <V3InvalidCaller as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V3InvalidCaller(decoded));
            }
            if let Ok(decoded) =
                <V3InvalidSwap as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V3InvalidSwap(decoded));
            }
            if let Ok(decoded) =
                <V3TooLittleReceived as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V3TooLittleReceived(decoded));
            }
            if let Ok(decoded) =
                <V3TooMuchRequested as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterErrors::V3TooMuchRequested(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for IUniversalRouterErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniversalRouterErrors::ContractLocked(element) => element.encode(),
                IUniversalRouterErrors::ETHNotAccepted(element) => element.encode(),
                IUniversalRouterErrors::ExecutionFailed(element) => element.encode(),
                IUniversalRouterErrors::FromAddressIsNotOwner(element) => element.encode(),
                IUniversalRouterErrors::InsufficientETH(element) => element.encode(),
                IUniversalRouterErrors::InsufficientToken(element) => element.encode(),
                IUniversalRouterErrors::InvalidBips(element) => element.encode(),
                IUniversalRouterErrors::InvalidCommandType(element) => element.encode(),
                IUniversalRouterErrors::InvalidOwnerERC1155(element) => element.encode(),
                IUniversalRouterErrors::InvalidOwnerERC721(element) => element.encode(),
                IUniversalRouterErrors::InvalidPath(element) => element.encode(),
                IUniversalRouterErrors::InvalidReserves(element) => element.encode(),
                IUniversalRouterErrors::LengthMismatch(element) => element.encode(),
                IUniversalRouterErrors::NoSlice(element) => element.encode(),
                IUniversalRouterErrors::SliceOutOfBounds(element) => element.encode(),
                IUniversalRouterErrors::SliceOverflow(element) => element.encode(),
                IUniversalRouterErrors::ToAddressOutOfBounds(element) => element.encode(),
                IUniversalRouterErrors::ToAddressOverflow(element) => element.encode(),
                IUniversalRouterErrors::ToUint24OutOfBounds(element) => element.encode(),
                IUniversalRouterErrors::ToUint24Overflow(element) => element.encode(),
                IUniversalRouterErrors::TransactionDeadlinePassed(element) => element.encode(),
                IUniversalRouterErrors::UnableToClaim(element) => element.encode(),
                IUniversalRouterErrors::UnsafeCast(element) => element.encode(),
                IUniversalRouterErrors::V2InvalidPath(element) => element.encode(),
                IUniversalRouterErrors::V2TooLittleReceived(element) => element.encode(),
                IUniversalRouterErrors::V2TooMuchRequested(element) => element.encode(),
                IUniversalRouterErrors::V3InvalidAmountOut(element) => element.encode(),
                IUniversalRouterErrors::V3InvalidCaller(element) => element.encode(),
                IUniversalRouterErrors::V3InvalidSwap(element) => element.encode(),
                IUniversalRouterErrors::V3TooLittleReceived(element) => element.encode(),
                IUniversalRouterErrors::V3TooMuchRequested(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniversalRouterErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniversalRouterErrors::ContractLocked(element) => element.fmt(f),
                IUniversalRouterErrors::ETHNotAccepted(element) => element.fmt(f),
                IUniversalRouterErrors::ExecutionFailed(element) => element.fmt(f),
                IUniversalRouterErrors::FromAddressIsNotOwner(element) => element.fmt(f),
                IUniversalRouterErrors::InsufficientETH(element) => element.fmt(f),
                IUniversalRouterErrors::InsufficientToken(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidBips(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidCommandType(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidOwnerERC1155(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidOwnerERC721(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidPath(element) => element.fmt(f),
                IUniversalRouterErrors::InvalidReserves(element) => element.fmt(f),
                IUniversalRouterErrors::LengthMismatch(element) => element.fmt(f),
                IUniversalRouterErrors::NoSlice(element) => element.fmt(f),
                IUniversalRouterErrors::SliceOutOfBounds(element) => element.fmt(f),
                IUniversalRouterErrors::SliceOverflow(element) => element.fmt(f),
                IUniversalRouterErrors::ToAddressOutOfBounds(element) => element.fmt(f),
                IUniversalRouterErrors::ToAddressOverflow(element) => element.fmt(f),
                IUniversalRouterErrors::ToUint24OutOfBounds(element) => element.fmt(f),
                IUniversalRouterErrors::ToUint24Overflow(element) => element.fmt(f),
                IUniversalRouterErrors::TransactionDeadlinePassed(element) => element.fmt(f),
                IUniversalRouterErrors::UnableToClaim(element) => element.fmt(f),
                IUniversalRouterErrors::UnsafeCast(element) => element.fmt(f),
                IUniversalRouterErrors::V2InvalidPath(element) => element.fmt(f),
                IUniversalRouterErrors::V2TooLittleReceived(element) => element.fmt(f),
                IUniversalRouterErrors::V2TooMuchRequested(element) => element.fmt(f),
                IUniversalRouterErrors::V3InvalidAmountOut(element) => element.fmt(f),
                IUniversalRouterErrors::V3InvalidCaller(element) => element.fmt(f),
                IUniversalRouterErrors::V3InvalidSwap(element) => element.fmt(f),
                IUniversalRouterErrors::V3TooLittleReceived(element) => element.fmt(f),
                IUniversalRouterErrors::V3TooMuchRequested(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ContractLocked> for IUniversalRouterErrors {
        fn from(var: ContractLocked) -> Self {
            IUniversalRouterErrors::ContractLocked(var)
        }
    }
    impl ::std::convert::From<ETHNotAccepted> for IUniversalRouterErrors {
        fn from(var: ETHNotAccepted) -> Self {
            IUniversalRouterErrors::ETHNotAccepted(var)
        }
    }
    impl ::std::convert::From<ExecutionFailed> for IUniversalRouterErrors {
        fn from(var: ExecutionFailed) -> Self {
            IUniversalRouterErrors::ExecutionFailed(var)
        }
    }
    impl ::std::convert::From<FromAddressIsNotOwner> for IUniversalRouterErrors {
        fn from(var: FromAddressIsNotOwner) -> Self {
            IUniversalRouterErrors::FromAddressIsNotOwner(var)
        }
    }
    impl ::std::convert::From<InsufficientETH> for IUniversalRouterErrors {
        fn from(var: InsufficientETH) -> Self {
            IUniversalRouterErrors::InsufficientETH(var)
        }
    }
    impl ::std::convert::From<InsufficientToken> for IUniversalRouterErrors {
        fn from(var: InsufficientToken) -> Self {
            IUniversalRouterErrors::InsufficientToken(var)
        }
    }
    impl ::std::convert::From<InvalidBips> for IUniversalRouterErrors {
        fn from(var: InvalidBips) -> Self {
            IUniversalRouterErrors::InvalidBips(var)
        }
    }
    impl ::std::convert::From<InvalidCommandType> for IUniversalRouterErrors {
        fn from(var: InvalidCommandType) -> Self {
            IUniversalRouterErrors::InvalidCommandType(var)
        }
    }
    impl ::std::convert::From<InvalidOwnerERC1155> for IUniversalRouterErrors {
        fn from(var: InvalidOwnerERC1155) -> Self {
            IUniversalRouterErrors::InvalidOwnerERC1155(var)
        }
    }
    impl ::std::convert::From<InvalidOwnerERC721> for IUniversalRouterErrors {
        fn from(var: InvalidOwnerERC721) -> Self {
            IUniversalRouterErrors::InvalidOwnerERC721(var)
        }
    }
    impl ::std::convert::From<InvalidPath> for IUniversalRouterErrors {
        fn from(var: InvalidPath) -> Self {
            IUniversalRouterErrors::InvalidPath(var)
        }
    }
    impl ::std::convert::From<InvalidReserves> for IUniversalRouterErrors {
        fn from(var: InvalidReserves) -> Self {
            IUniversalRouterErrors::InvalidReserves(var)
        }
    }
    impl ::std::convert::From<LengthMismatch> for IUniversalRouterErrors {
        fn from(var: LengthMismatch) -> Self {
            IUniversalRouterErrors::LengthMismatch(var)
        }
    }
    impl ::std::convert::From<NoSlice> for IUniversalRouterErrors {
        fn from(var: NoSlice) -> Self {
            IUniversalRouterErrors::NoSlice(var)
        }
    }
    impl ::std::convert::From<SliceOutOfBounds> for IUniversalRouterErrors {
        fn from(var: SliceOutOfBounds) -> Self {
            IUniversalRouterErrors::SliceOutOfBounds(var)
        }
    }
    impl ::std::convert::From<SliceOverflow> for IUniversalRouterErrors {
        fn from(var: SliceOverflow) -> Self {
            IUniversalRouterErrors::SliceOverflow(var)
        }
    }
    impl ::std::convert::From<ToAddressOutOfBounds> for IUniversalRouterErrors {
        fn from(var: ToAddressOutOfBounds) -> Self {
            IUniversalRouterErrors::ToAddressOutOfBounds(var)
        }
    }
    impl ::std::convert::From<ToAddressOverflow> for IUniversalRouterErrors {
        fn from(var: ToAddressOverflow) -> Self {
            IUniversalRouterErrors::ToAddressOverflow(var)
        }
    }
    impl ::std::convert::From<ToUint24OutOfBounds> for IUniversalRouterErrors {
        fn from(var: ToUint24OutOfBounds) -> Self {
            IUniversalRouterErrors::ToUint24OutOfBounds(var)
        }
    }
    impl ::std::convert::From<ToUint24Overflow> for IUniversalRouterErrors {
        fn from(var: ToUint24Overflow) -> Self {
            IUniversalRouterErrors::ToUint24Overflow(var)
        }
    }
    impl ::std::convert::From<TransactionDeadlinePassed> for IUniversalRouterErrors {
        fn from(var: TransactionDeadlinePassed) -> Self {
            IUniversalRouterErrors::TransactionDeadlinePassed(var)
        }
    }
    impl ::std::convert::From<UnableToClaim> for IUniversalRouterErrors {
        fn from(var: UnableToClaim) -> Self {
            IUniversalRouterErrors::UnableToClaim(var)
        }
    }
    impl ::std::convert::From<UnsafeCast> for IUniversalRouterErrors {
        fn from(var: UnsafeCast) -> Self {
            IUniversalRouterErrors::UnsafeCast(var)
        }
    }
    impl ::std::convert::From<V2InvalidPath> for IUniversalRouterErrors {
        fn from(var: V2InvalidPath) -> Self {
            IUniversalRouterErrors::V2InvalidPath(var)
        }
    }
    impl ::std::convert::From<V2TooLittleReceived> for IUniversalRouterErrors {
        fn from(var: V2TooLittleReceived) -> Self {
            IUniversalRouterErrors::V2TooLittleReceived(var)
        }
    }
    impl ::std::convert::From<V2TooMuchRequested> for IUniversalRouterErrors {
        fn from(var: V2TooMuchRequested) -> Self {
            IUniversalRouterErrors::V2TooMuchRequested(var)
        }
    }
    impl ::std::convert::From<V3InvalidAmountOut> for IUniversalRouterErrors {
        fn from(var: V3InvalidAmountOut) -> Self {
            IUniversalRouterErrors::V3InvalidAmountOut(var)
        }
    }
    impl ::std::convert::From<V3InvalidCaller> for IUniversalRouterErrors {
        fn from(var: V3InvalidCaller) -> Self {
            IUniversalRouterErrors::V3InvalidCaller(var)
        }
    }
    impl ::std::convert::From<V3InvalidSwap> for IUniversalRouterErrors {
        fn from(var: V3InvalidSwap) -> Self {
            IUniversalRouterErrors::V3InvalidSwap(var)
        }
    }
    impl ::std::convert::From<V3TooLittleReceived> for IUniversalRouterErrors {
        fn from(var: V3TooLittleReceived) -> Self {
            IUniversalRouterErrors::V3TooLittleReceived(var)
        }
    }
    impl ::std::convert::From<V3TooMuchRequested> for IUniversalRouterErrors {
        fn from(var: V3TooMuchRequested) -> Self {
            IUniversalRouterErrors::V3TooMuchRequested(var)
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
    #[ethevent(name = "RewardsSent", abi = "RewardsSent(uint256)")]
    pub struct RewardsSentFilter {
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `collectRewards` function with signature `collectRewards(bytes)` and selector `[112, 154, 28, 194]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collectRewards", abi = "collectRewards(bytes)")]
    pub struct CollectRewardsCall {
        pub looks_rare_claim: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[])` and selector `[36, 133, 107, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[])")]
    pub struct ExecuteCall {
        pub commands: ethers_core::types::Bytes,
        pub inputs: ::std::vec::Vec<ethers_core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[],uint256)` and selector `[53, 147, 86, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[],uint256)")]
    pub struct ExecuteWithCommandsAndInputsCall {
        pub commands: ethers_core::types::Bytes,
        pub inputs: ::std::vec::Vec<ethers_core::types::Bytes>,
        pub deadline: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ethers_core::types::Address,
        pub ethers_core::types::Address,
        pub ::std::vec::Vec<ethers_core::types::U256>,
        pub ::std::vec::Vec<ethers_core::types::U256>,
        pub ethers_core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ethers_core::types::Address,
        pub ethers_core::types::Address,
        pub ethers_core::types::U256,
        pub ethers_core::types::U256,
        pub ethers_core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `[21, 11, 122, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "onERC721Received", abi = "onERC721Received(address,address,uint256,bytes)")]
    pub struct OnERC721ReceivedCall(
        pub ethers_core::types::Address,
        pub ethers_core::types::Address,
        pub ethers_core::types::U256,
        pub ethers_core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
    pub enum IUniversalRouterCalls {
        CollectRewards(CollectRewardsCall),
        Execute(ExecuteCall),
        ExecuteWithCommandsAndInputs(ExecuteWithCommandsAndInputsCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        SupportsInterface(SupportsInterfaceCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ethers_core::abi::AbiDecode for IUniversalRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers_core::abi::AbiError> {
            if let Ok(decoded) =
                <CollectRewardsCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::CollectRewards(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithCommandsAndInputsCall as ethers_core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniversalRouterCalls::ExecuteWithCommandsAndInputs(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::OnERC721Received(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCalls::UniswapV3SwapCallback(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for IUniversalRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniversalRouterCalls::CollectRewards(element) => element.encode(),
                IUniversalRouterCalls::Execute(element) => element.encode(),
                IUniversalRouterCalls::ExecuteWithCommandsAndInputs(element) => element.encode(),
                IUniversalRouterCalls::OnERC1155BatchReceived(element) => element.encode(),
                IUniversalRouterCalls::OnERC1155Received(element) => element.encode(),
                IUniversalRouterCalls::OnERC721Received(element) => element.encode(),
                IUniversalRouterCalls::SupportsInterface(element) => element.encode(),
                IUniversalRouterCalls::UniswapV3SwapCallback(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniversalRouterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniversalRouterCalls::CollectRewards(element) => element.fmt(f),
                IUniversalRouterCalls::Execute(element) => element.fmt(f),
                IUniversalRouterCalls::ExecuteWithCommandsAndInputs(element) => element.fmt(f),
                IUniversalRouterCalls::OnERC1155BatchReceived(element) => element.fmt(f),
                IUniversalRouterCalls::OnERC1155Received(element) => element.fmt(f),
                IUniversalRouterCalls::OnERC721Received(element) => element.fmt(f),
                IUniversalRouterCalls::SupportsInterface(element) => element.fmt(f),
                IUniversalRouterCalls::UniswapV3SwapCallback(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CollectRewardsCall> for IUniversalRouterCalls {
        fn from(var: CollectRewardsCall) -> Self {
            IUniversalRouterCalls::CollectRewards(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for IUniversalRouterCalls {
        fn from(var: ExecuteCall) -> Self {
            IUniversalRouterCalls::Execute(var)
        }
    }
    impl ::std::convert::From<ExecuteWithCommandsAndInputsCall> for IUniversalRouterCalls {
        fn from(var: ExecuteWithCommandsAndInputsCall) -> Self {
            IUniversalRouterCalls::ExecuteWithCommandsAndInputs(var)
        }
    }
    impl ::std::convert::From<OnERC1155BatchReceivedCall> for IUniversalRouterCalls {
        fn from(var: OnERC1155BatchReceivedCall) -> Self {
            IUniversalRouterCalls::OnERC1155BatchReceived(var)
        }
    }
    impl ::std::convert::From<OnERC1155ReceivedCall> for IUniversalRouterCalls {
        fn from(var: OnERC1155ReceivedCall) -> Self {
            IUniversalRouterCalls::OnERC1155Received(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for IUniversalRouterCalls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            IUniversalRouterCalls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for IUniversalRouterCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            IUniversalRouterCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UniswapV3SwapCallbackCall> for IUniversalRouterCalls {
        fn from(var: UniswapV3SwapCallbackCall) -> Self {
            IUniversalRouterCalls::UniswapV3SwapCallback(var)
        }
    }
    #[doc = "Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `[21, 11, 122, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
