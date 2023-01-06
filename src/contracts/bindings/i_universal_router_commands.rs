pub use i_universal_router_commands::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_universal_router_commands {
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
    #[doc = "IUniversalRouterCommands was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"punkId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"CryptoPunks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Foundation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"LooksRare1155\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"LooksRare721\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Nft20\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Nftx\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"OwnerCheck1155\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"OwnerCheck721\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bips\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"PayPortion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IAllowanceTransfer.PermitSingle\",\"name\":\"permitSingle\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IAllowanceTransfer.PermitDetails\",\"name\":\"details\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"expiration\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"nonce\",\"type\":\"uint48\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"sigDeadline\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Permit2Permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IAllowanceTransfer.PermitBatch\",\"name\":\"permitBatch\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IAllowanceTransfer.PermitDetails[]\",\"name\":\"details\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"expiration\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"nonce\",\"type\":\"uint48\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"sigDeadline\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Permit2PermitBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Permit2TransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IAllowanceTransfer.AllowanceTransferDetails[]\",\"name\":\"batchDetails\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Permit2TransferFromBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Seaport\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"SudoSwap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Sweep\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"SweepErc1155\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"SweepErc721\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"Transfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"UnwrapEth\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"V2SwapExactIn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"V2SwapExactOut\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"V3SwapExactIn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"V3SwapExactOut\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"WrapEth\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"X2y21155\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"X2y2721\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IUNIVERSALROUTERCOMMANDS_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            ethers_core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IUniversalRouterCommands<M>(ethers_contract::Contract<M>);
    impl<M> Clone for IUniversalRouterCommands<M> {
        fn clone(&self) -> Self {
            IUniversalRouterCommands(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniversalRouterCommands<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniversalRouterCommands<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniversalRouterCommands)).field(&self.address()).finish()
        }
    }
    impl<M: ethers_providers::Middleware> IUniversalRouterCommands<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers_contract::Contract::new(
                address.into(),
                IUNIVERSALROUTERCOMMANDS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `CryptoPunks` (0x75f8cf32) function"]
        pub fn crypto_punks(
            &self,
            punk_id: ethers_core::types::U256,
            recipient: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 248, 207, 50], (punk_id, recipient, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Foundation` (0x3bdef733) function"]
        pub fn foundation(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 222, 247, 51], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LooksRare1155` (0xa337dfa0) function"]
        pub fn looks_rare_1155(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 55, 223, 160], (value, data, recipient, token, id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LooksRare721` (0x4adcff0f) function"]
        pub fn looks_rare_721(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 220, 255, 15], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Nft20` (0x099841d4) function"]
        pub fn nft_20(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 152, 65, 212], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Nftx` (0x44f7c1e2) function"]
        pub fn nftx(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 247, 193, 226], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `OwnerCheck1155` (0x0d574dd0) function"]
        pub fn owner_check_1155(
            &self,
            owner: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 87, 77, 208], (owner, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `OwnerCheck721` (0x44ae5a7d) function"]
        pub fn owner_check_721(
            &self,
            owner: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 174, 90, 125], (owner, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PayPortion` (0xdbed2b0b) function"]
        pub fn pay_portion(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            bips: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 237, 43, 11], (token, recipient, bips))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Permit2Permit` (0xaafbb189) function"]
        pub fn permit_2_permit(
            &self,
            permit_single: PermitSingle,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 251, 177, 137], (permit_single, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Permit2PermitBatch` (0xe1b9e849) function"]
        pub fn permit_2_permit_batch(
            &self,
            permit_batch: PermitBatch,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 185, 232, 73], (permit_batch, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Permit2TransferFrom` (0xaa2ab1aa) function"]
        pub fn permit_2_transfer_from(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 42, 177, 170], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Permit2TransferFromBatch` (0xaa8ea976) function"]
        pub fn permit_2_transfer_from_batch(
            &self,
            batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 142, 169, 118], batch_details)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Seaport` (0xe20ffbf6) function"]
        pub fn seaport(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 15, 251, 246], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SudoSwap` (0x3ac0f445) function"]
        pub fn sudo_swap(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 192, 244, 69], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Sweep` (0xed679328) function"]
        pub fn sweep(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 103, 147, 40], (token, recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SweepErc1155` (0xbc5a8260) function"]
        pub fn sweep_erc_1155(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 90, 130, 96], (token, recipient, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SweepErc721` (0xf34604aa) function"]
        pub fn sweep_erc_721(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 70, 4, 170], (token, recipient, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Transfer` (0xddf252ad) function"]
        pub fn transfer(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 242, 82, 173], (token, recipient, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UnwrapEth` (0xdc13a81f) function"]
        pub fn unwrap_eth(
            &self,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 19, 168, 31], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `V2SwapExactIn` (0x7b3c1767) function"]
        pub fn v2_swap_exact_in(
            &self,
            recipient: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: ::std::vec::Vec<ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 60, 23, 103],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `V2SwapExactOut` (0xf6ec5868) function"]
        pub fn v2_swap_exact_out(
            &self,
            recipient: ethers_core::types::Address,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: ::std::vec::Vec<ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [246, 236, 88, 104],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `V3SwapExactIn` (0xfa920a5d) function"]
        pub fn v3_swap_exact_in(
            &self,
            recipient: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [250, 146, 10, 93],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `V3SwapExactOut` (0x2c9b3a91) function"]
        pub fn v3_swap_exact_out(
            &self,
            recipient: ethers_core::types::Address,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 155, 58, 145],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WrapEth` (0xfae3e7b4) function"]
        pub fn wrap_eth(
            &self,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 227, 231, 180], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `X2y21155` (0x613afcee) function"]
        pub fn x_2y_21155(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 58, 252, 238], (value, data, recipient, token, id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `X2y2721` (0x05059058) function"]
        pub fn x_2y_2721(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 5, 144, 88], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers_providers::Middleware> From<ethers_contract::Contract<M>>
        for IUniversalRouterCommands<M>
    {
        fn from(contract: ethers_contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `CryptoPunks` function with signature `CryptoPunks(uint256,address,uint256)` and selector `[117, 248, 207, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "CryptoPunks", abi = "CryptoPunks(uint256,address,uint256)")]
    pub struct CryptoPunksCall {
        pub punk_id: ethers_core::types::U256,
        pub recipient: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Foundation` function with signature `Foundation(uint256,bytes,address,address,uint256)` and selector `[59, 222, 247, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Foundation", abi = "Foundation(uint256,bytes,address,address,uint256)")]
    pub struct FoundationCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `LooksRare1155` function with signature `LooksRare1155(uint256,bytes,address,address,uint256,uint256)` and selector `[163, 55, 223, 160]`"]
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
        name = "LooksRare1155",
        abi = "LooksRare1155(uint256,bytes,address,address,uint256,uint256)"
    )]
    pub struct LooksRare1155Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `LooksRare721` function with signature `LooksRare721(uint256,bytes,address,address,uint256)` and selector `[74, 220, 255, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "LooksRare721", abi = "LooksRare721(uint256,bytes,address,address,uint256)")]
    pub struct LooksRare721Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Nft20` function with signature `Nft20(uint256,bytes)` and selector `[9, 152, 65, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Nft20", abi = "Nft20(uint256,bytes)")]
    pub struct Nft20Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `Nftx` function with signature `Nftx(uint256,bytes)` and selector `[68, 247, 193, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Nftx", abi = "Nftx(uint256,bytes)")]
    pub struct NftxCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `OwnerCheck1155` function with signature `OwnerCheck1155(address,address,uint256)` and selector `[13, 87, 77, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "OwnerCheck1155", abi = "OwnerCheck1155(address,address,uint256)")]
    pub struct OwnerCheck1155Call {
        pub owner: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `OwnerCheck721` function with signature `OwnerCheck721(address,address,uint256)` and selector `[68, 174, 90, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "OwnerCheck721", abi = "OwnerCheck721(address,address,uint256)")]
    pub struct OwnerCheck721Call {
        pub owner: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `PayPortion` function with signature `PayPortion(address,address,uint256)` and selector `[219, 237, 43, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "PayPortion", abi = "PayPortion(address,address,uint256)")]
    pub struct PayPortionCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub bips: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Permit2Permit` function with signature `Permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)` and selector `[170, 251, 177, 137]`"]
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
        name = "Permit2Permit",
        abi = "Permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)"
    )]
    pub struct Permit2PermitCall {
        pub permit_single: PermitSingle,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `Permit2PermitBatch` function with signature `Permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)` and selector `[225, 185, 232, 73]`"]
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
        name = "Permit2PermitBatch",
        abi = "Permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)"
    )]
    pub struct Permit2PermitBatchCall {
        pub permit_batch: PermitBatch,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `Permit2TransferFrom` function with signature `Permit2TransferFrom(address,address,uint256)` and selector `[170, 42, 177, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Permit2TransferFrom", abi = "Permit2TransferFrom(address,address,uint256)")]
    pub struct Permit2TransferFromCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Permit2TransferFromBatch` function with signature `Permit2TransferFromBatch((address,address,uint160,address)[])` and selector `[170, 142, 169, 118]`"]
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
        name = "Permit2TransferFromBatch",
        abi = "Permit2TransferFromBatch((address,address,uint160,address)[])"
    )]
    pub struct Permit2TransferFromBatchCall {
        pub batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
    }
    #[doc = "Container type for all input parameters for the `Seaport` function with signature `Seaport(uint256,bytes)` and selector `[226, 15, 251, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Seaport", abi = "Seaport(uint256,bytes)")]
    pub struct SeaportCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `SudoSwap` function with signature `SudoSwap(uint256,bytes)` and selector `[58, 192, 244, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SudoSwap", abi = "SudoSwap(uint256,bytes)")]
    pub struct SudoSwapCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `Sweep` function with signature `Sweep(address,address,uint256)` and selector `[237, 103, 147, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Sweep", abi = "Sweep(address,address,uint256)")]
    pub struct SweepCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `SweepErc1155` function with signature `SweepErc1155(address,address,uint256)` and selector `[188, 90, 130, 96]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SweepErc1155", abi = "SweepErc1155(address,address,uint256)")]
    pub struct SweepErc1155Call {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `SweepErc721` function with signature `SweepErc721(address,address,uint256)` and selector `[243, 70, 4, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "SweepErc721", abi = "SweepErc721(address,address,uint256)")]
    pub struct SweepErc721Call {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Transfer` function with signature `Transfer(address,address,uint256)` and selector `[221, 242, 82, 173]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `UnwrapEth` function with signature `UnwrapEth(address,uint256)` and selector `[220, 19, 168, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "UnwrapEth", abi = "UnwrapEth(address,uint256)")]
    pub struct UnwrapEthCall {
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `V2SwapExactIn` function with signature `V2SwapExactIn(address,uint256,uint256,address[],bool)` and selector `[123, 60, 23, 103]`"]
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
        name = "V2SwapExactIn",
        abi = "V2SwapExactIn(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactInCall {
        pub recipient: ethers_core::types::Address,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_min: ethers_core::types::U256,
        pub path: ::std::vec::Vec<ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `V2SwapExactOut` function with signature `V2SwapExactOut(address,uint256,uint256,address[],bool)` and selector `[246, 236, 88, 104]`"]
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
        name = "V2SwapExactOut",
        abi = "V2SwapExactOut(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactOutCall {
        pub recipient: ethers_core::types::Address,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_max: ethers_core::types::U256,
        pub path: ::std::vec::Vec<ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `V3SwapExactIn` function with signature `V3SwapExactIn(address,uint256,uint256,bytes,bool)` and selector `[250, 146, 10, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "V3SwapExactIn", abi = "V3SwapExactIn(address,uint256,uint256,bytes,bool)")]
    pub struct V3SwapExactInCall {
        pub recipient: ethers_core::types::Address,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_min: ethers_core::types::U256,
        pub path: ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `V3SwapExactOut` function with signature `V3SwapExactOut(address,uint256,uint256,bytes,bool)` and selector `[44, 155, 58, 145]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "V3SwapExactOut", abi = "V3SwapExactOut(address,uint256,uint256,bytes,bool)")]
    pub struct V3SwapExactOutCall {
        pub recipient: ethers_core::types::Address,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_max: ethers_core::types::U256,
        pub path: ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `WrapEth` function with signature `WrapEth(address,uint256)` and selector `[250, 227, 231, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "WrapEth", abi = "WrapEth(address,uint256)")]
    pub struct WrapEthCall {
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `X2y21155` function with signature `X2y21155(uint256,bytes,address,address,uint256,uint256)` and selector `[97, 58, 252, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "X2y21155", abi = "X2y21155(uint256,bytes,address,address,uint256,uint256)")]
    pub struct X2Y21155Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `X2y2721` function with signature `X2y2721(uint256,bytes,address,address,uint256)` and selector `[5, 5, 144, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "X2y2721", abi = "X2y2721(uint256,bytes,address,address,uint256)")]
    pub struct X2Y2721Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum IUniversalRouterCommandsCalls {
        CryptoPunks(CryptoPunksCall),
        Foundation(FoundationCall),
        LooksRare1155(LooksRare1155Call),
        LooksRare721(LooksRare721Call),
        Nft20(Nft20Call),
        Nftx(NftxCall),
        OwnerCheck1155(OwnerCheck1155Call),
        OwnerCheck721(OwnerCheck721Call),
        PayPortion(PayPortionCall),
        Permit2Permit(Permit2PermitCall),
        Permit2PermitBatch(Permit2PermitBatchCall),
        Permit2TransferFrom(Permit2TransferFromCall),
        Permit2TransferFromBatch(Permit2TransferFromBatchCall),
        Seaport(SeaportCall),
        SudoSwap(SudoSwapCall),
        Sweep(SweepCall),
        SweepErc1155(SweepErc1155Call),
        SweepErc721(SweepErc721Call),
        Transfer(TransferCall),
        UnwrapEth(UnwrapEthCall),
        V2SwapExactIn(V2SwapExactInCall),
        V2SwapExactOut(V2SwapExactOutCall),
        V3SwapExactIn(V3SwapExactInCall),
        V3SwapExactOut(V3SwapExactOutCall),
        WrapEth(WrapEthCall),
        X2Y21155(X2Y21155Call),
        X2Y2721(X2Y2721Call),
    }
    impl ethers_core::abi::AbiDecode for IUniversalRouterCommandsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers_core::abi::AbiError> {
            if let Ok(decoded) =
                <CryptoPunksCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::CryptoPunks(decoded))
            }
            if let Ok(decoded) =
                <FoundationCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Foundation(decoded))
            }
            if let Ok(decoded) =
                <LooksRare1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::LooksRare1155(decoded))
            }
            if let Ok(decoded) =
                <LooksRare721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::LooksRare721(decoded))
            }
            if let Ok(decoded) = <Nft20Call as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Nft20(decoded))
            }
            if let Ok(decoded) = <NftxCall as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Nftx(decoded))
            }
            if let Ok(decoded) =
                <OwnerCheck1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::OwnerCheck1155(decoded))
            }
            if let Ok(decoded) =
                <OwnerCheck721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::OwnerCheck721(decoded))
            }
            if let Ok(decoded) =
                <PayPortionCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::PayPortion(decoded))
            }
            if let Ok(decoded) =
                <Permit2PermitCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2Permit(decoded))
            }
            if let Ok(decoded) =
                <Permit2PermitBatchCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2PermitBatch(decoded))
            }
            if let Ok(decoded) =
                <Permit2TransferFromCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2TransferFrom(decoded))
            }
            if let Ok(decoded) =
                <Permit2TransferFromBatchCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2TransferFromBatch(decoded))
            }
            if let Ok(decoded) = <SeaportCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Seaport(decoded))
            }
            if let Ok(decoded) =
                <SudoSwapCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::SudoSwap(decoded))
            }
            if let Ok(decoded) = <SweepCall as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Sweep(decoded))
            }
            if let Ok(decoded) =
                <SweepErc1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::SweepErc1155(decoded))
            }
            if let Ok(decoded) =
                <SweepErc721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::SweepErc721(decoded))
            }
            if let Ok(decoded) =
                <TransferCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Transfer(decoded))
            }
            if let Ok(decoded) =
                <UnwrapEthCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::UnwrapEth(decoded))
            }
            if let Ok(decoded) =
                <V2SwapExactInCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V2SwapExactIn(decoded))
            }
            if let Ok(decoded) =
                <V2SwapExactOutCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V2SwapExactOut(decoded))
            }
            if let Ok(decoded) =
                <V3SwapExactInCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V3SwapExactIn(decoded))
            }
            if let Ok(decoded) =
                <V3SwapExactOutCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V3SwapExactOut(decoded))
            }
            if let Ok(decoded) = <WrapEthCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::WrapEth(decoded))
            }
            if let Ok(decoded) =
                <X2Y21155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::X2Y21155(decoded))
            }
            if let Ok(decoded) = <X2Y2721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::X2Y2721(decoded))
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for IUniversalRouterCommandsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniversalRouterCommandsCalls::CryptoPunks(element) => element.encode(),
                IUniversalRouterCommandsCalls::Foundation(element) => element.encode(),
                IUniversalRouterCommandsCalls::LooksRare1155(element) => element.encode(),
                IUniversalRouterCommandsCalls::LooksRare721(element) => element.encode(),
                IUniversalRouterCommandsCalls::Nft20(element) => element.encode(),
                IUniversalRouterCommandsCalls::Nftx(element) => element.encode(),
                IUniversalRouterCommandsCalls::OwnerCheck1155(element) => element.encode(),
                IUniversalRouterCommandsCalls::OwnerCheck721(element) => element.encode(),
                IUniversalRouterCommandsCalls::PayPortion(element) => element.encode(),
                IUniversalRouterCommandsCalls::Permit2Permit(element) => element.encode(),
                IUniversalRouterCommandsCalls::Permit2PermitBatch(element) => element.encode(),
                IUniversalRouterCommandsCalls::Permit2TransferFrom(element) => element.encode(),
                IUniversalRouterCommandsCalls::Permit2TransferFromBatch(element) => {
                    element.encode()
                }
                IUniversalRouterCommandsCalls::Seaport(element) => element.encode(),
                IUniversalRouterCommandsCalls::SudoSwap(element) => element.encode(),
                IUniversalRouterCommandsCalls::Sweep(element) => element.encode(),
                IUniversalRouterCommandsCalls::SweepErc1155(element) => element.encode(),
                IUniversalRouterCommandsCalls::SweepErc721(element) => element.encode(),
                IUniversalRouterCommandsCalls::Transfer(element) => element.encode(),
                IUniversalRouterCommandsCalls::UnwrapEth(element) => element.encode(),
                IUniversalRouterCommandsCalls::V2SwapExactIn(element) => element.encode(),
                IUniversalRouterCommandsCalls::V2SwapExactOut(element) => element.encode(),
                IUniversalRouterCommandsCalls::V3SwapExactIn(element) => element.encode(),
                IUniversalRouterCommandsCalls::V3SwapExactOut(element) => element.encode(),
                IUniversalRouterCommandsCalls::WrapEth(element) => element.encode(),
                IUniversalRouterCommandsCalls::X2Y21155(element) => element.encode(),
                IUniversalRouterCommandsCalls::X2Y2721(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniversalRouterCommandsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniversalRouterCommandsCalls::CryptoPunks(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Foundation(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::LooksRare1155(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::LooksRare721(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Nft20(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Nftx(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::OwnerCheck1155(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::OwnerCheck721(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::PayPortion(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Permit2Permit(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Permit2PermitBatch(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Permit2TransferFrom(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Permit2TransferFromBatch(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Seaport(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::SudoSwap(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Sweep(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::SweepErc1155(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::SweepErc721(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Transfer(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::UnwrapEth(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::V2SwapExactIn(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::V2SwapExactOut(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::V3SwapExactIn(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::V3SwapExactOut(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::WrapEth(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::X2Y21155(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::X2Y2721(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CryptoPunksCall> for IUniversalRouterCommandsCalls {
        fn from(var: CryptoPunksCall) -> Self {
            IUniversalRouterCommandsCalls::CryptoPunks(var)
        }
    }
    impl ::std::convert::From<FoundationCall> for IUniversalRouterCommandsCalls {
        fn from(var: FoundationCall) -> Self {
            IUniversalRouterCommandsCalls::Foundation(var)
        }
    }
    impl ::std::convert::From<LooksRare1155Call> for IUniversalRouterCommandsCalls {
        fn from(var: LooksRare1155Call) -> Self {
            IUniversalRouterCommandsCalls::LooksRare1155(var)
        }
    }
    impl ::std::convert::From<LooksRare721Call> for IUniversalRouterCommandsCalls {
        fn from(var: LooksRare721Call) -> Self {
            IUniversalRouterCommandsCalls::LooksRare721(var)
        }
    }
    impl ::std::convert::From<Nft20Call> for IUniversalRouterCommandsCalls {
        fn from(var: Nft20Call) -> Self {
            IUniversalRouterCommandsCalls::Nft20(var)
        }
    }
    impl ::std::convert::From<NftxCall> for IUniversalRouterCommandsCalls {
        fn from(var: NftxCall) -> Self {
            IUniversalRouterCommandsCalls::Nftx(var)
        }
    }
    impl ::std::convert::From<OwnerCheck1155Call> for IUniversalRouterCommandsCalls {
        fn from(var: OwnerCheck1155Call) -> Self {
            IUniversalRouterCommandsCalls::OwnerCheck1155(var)
        }
    }
    impl ::std::convert::From<OwnerCheck721Call> for IUniversalRouterCommandsCalls {
        fn from(var: OwnerCheck721Call) -> Self {
            IUniversalRouterCommandsCalls::OwnerCheck721(var)
        }
    }
    impl ::std::convert::From<PayPortionCall> for IUniversalRouterCommandsCalls {
        fn from(var: PayPortionCall) -> Self {
            IUniversalRouterCommandsCalls::PayPortion(var)
        }
    }
    impl ::std::convert::From<Permit2PermitCall> for IUniversalRouterCommandsCalls {
        fn from(var: Permit2PermitCall) -> Self {
            IUniversalRouterCommandsCalls::Permit2Permit(var)
        }
    }
    impl ::std::convert::From<Permit2PermitBatchCall> for IUniversalRouterCommandsCalls {
        fn from(var: Permit2PermitBatchCall) -> Self {
            IUniversalRouterCommandsCalls::Permit2PermitBatch(var)
        }
    }
    impl ::std::convert::From<Permit2TransferFromCall> for IUniversalRouterCommandsCalls {
        fn from(var: Permit2TransferFromCall) -> Self {
            IUniversalRouterCommandsCalls::Permit2TransferFrom(var)
        }
    }
    impl ::std::convert::From<Permit2TransferFromBatchCall> for IUniversalRouterCommandsCalls {
        fn from(var: Permit2TransferFromBatchCall) -> Self {
            IUniversalRouterCommandsCalls::Permit2TransferFromBatch(var)
        }
    }
    impl ::std::convert::From<SeaportCall> for IUniversalRouterCommandsCalls {
        fn from(var: SeaportCall) -> Self {
            IUniversalRouterCommandsCalls::Seaport(var)
        }
    }
    impl ::std::convert::From<SudoSwapCall> for IUniversalRouterCommandsCalls {
        fn from(var: SudoSwapCall) -> Self {
            IUniversalRouterCommandsCalls::SudoSwap(var)
        }
    }
    impl ::std::convert::From<SweepCall> for IUniversalRouterCommandsCalls {
        fn from(var: SweepCall) -> Self {
            IUniversalRouterCommandsCalls::Sweep(var)
        }
    }
    impl ::std::convert::From<SweepErc1155Call> for IUniversalRouterCommandsCalls {
        fn from(var: SweepErc1155Call) -> Self {
            IUniversalRouterCommandsCalls::SweepErc1155(var)
        }
    }
    impl ::std::convert::From<SweepErc721Call> for IUniversalRouterCommandsCalls {
        fn from(var: SweepErc721Call) -> Self {
            IUniversalRouterCommandsCalls::SweepErc721(var)
        }
    }
    impl ::std::convert::From<TransferCall> for IUniversalRouterCommandsCalls {
        fn from(var: TransferCall) -> Self {
            IUniversalRouterCommandsCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<UnwrapEthCall> for IUniversalRouterCommandsCalls {
        fn from(var: UnwrapEthCall) -> Self {
            IUniversalRouterCommandsCalls::UnwrapEth(var)
        }
    }
    impl ::std::convert::From<V2SwapExactInCall> for IUniversalRouterCommandsCalls {
        fn from(var: V2SwapExactInCall) -> Self {
            IUniversalRouterCommandsCalls::V2SwapExactIn(var)
        }
    }
    impl ::std::convert::From<V2SwapExactOutCall> for IUniversalRouterCommandsCalls {
        fn from(var: V2SwapExactOutCall) -> Self {
            IUniversalRouterCommandsCalls::V2SwapExactOut(var)
        }
    }
    impl ::std::convert::From<V3SwapExactInCall> for IUniversalRouterCommandsCalls {
        fn from(var: V3SwapExactInCall) -> Self {
            IUniversalRouterCommandsCalls::V3SwapExactIn(var)
        }
    }
    impl ::std::convert::From<V3SwapExactOutCall> for IUniversalRouterCommandsCalls {
        fn from(var: V3SwapExactOutCall) -> Self {
            IUniversalRouterCommandsCalls::V3SwapExactOut(var)
        }
    }
    impl ::std::convert::From<WrapEthCall> for IUniversalRouterCommandsCalls {
        fn from(var: WrapEthCall) -> Self {
            IUniversalRouterCommandsCalls::WrapEth(var)
        }
    }
    impl ::std::convert::From<X2Y21155Call> for IUniversalRouterCommandsCalls {
        fn from(var: X2Y21155Call) -> Self {
            IUniversalRouterCommandsCalls::X2Y21155(var)
        }
    }
    impl ::std::convert::From<X2Y2721Call> for IUniversalRouterCommandsCalls {
        fn from(var: X2Y2721Call) -> Self {
            IUniversalRouterCommandsCalls::X2Y2721(var)
        }
    }
    #[doc = "`AllowanceTransferDetails(address,address,uint160,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct AllowanceTransferDetails {
        pub from: ethers_core::types::Address,
        pub to: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub token: ethers_core::types::Address,
    }
    #[doc = "`PermitBatch((address,uint160,uint48,uint48)[],address,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct PermitBatch {
        pub details: ::std::vec::Vec<PermitDetails>,
        pub spender: ethers_core::types::Address,
        pub sig_deadline: ethers_core::types::U256,
    }
    #[doc = "`PermitDetails(address,uint160,uint48,uint48)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct PermitDetails {
        pub token: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub expiration: u64,
        pub nonce: u64,
    }
    #[doc = "`PermitSingle((address,uint160,uint48,uint48),address,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers_contract :: EthAbiType,
        ethers_contract :: EthAbiCodec,
    )]
    pub struct PermitSingle {
        pub details: PermitDetails,
        pub spender: ethers_core::types::Address,
        pub sig_deadline: ethers_core::types::U256,
    }
}
