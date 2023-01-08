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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"punkId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"cryptopunks\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"foundation\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"looksRare1155\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"looksRare721\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"nft20\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"nftx\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minBalance\",\"type\":\"uint256\"}],\"name\":\"ownerCheck1155\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"ownerCheck721\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"bips\",\"type\":\"uint256\"}],\"name\":\"payPortion\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\"},{\"internalType\":\"uint48\",\"name\":\"expiration\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"nonce\",\"type\":\"uint48\"}],\"internalType\":\"struct IAllowanceTransfer.PermitDetails\",\"name\":\"details\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sigDeadline\",\"type\":\"uint256\"}],\"internalType\":\"struct IAllowanceTransfer.PermitSingle\",\"name\":\"permitSingle\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"permit2Permit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\"},{\"internalType\":\"uint48\",\"name\":\"expiration\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"nonce\",\"type\":\"uint48\"}],\"internalType\":\"struct IAllowanceTransfer.PermitDetails[]\",\"name\":\"details\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sigDeadline\",\"type\":\"uint256\"}],\"internalType\":\"struct IAllowanceTransfer.PermitBatch\",\"name\":\"permitBatch\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"permit2PermitBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\"}],\"name\":\"permit2TransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint160\",\"name\":\"amount\",\"type\":\"uint160\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"internalType\":\"struct IAllowanceTransfer.AllowanceTransferDetails[]\",\"name\":\"batchDetails\",\"type\":\"tuple[]\"}],\"name\":\"permit2TransferFromBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"seaport\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"sudoswap\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\"}],\"name\":\"sweep\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"sweepErc1155\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"sweepErc721\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\"}],\"name\":\"unwrapWeth\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\"}],\"name\":\"v2SwapExactIn\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\"}],\"name\":\"v2SwapExactOut\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\"}],\"name\":\"v3SwapExactIn\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"bool\",\"name\":\"payerIsUser\",\"type\":\"bool\"}],\"name\":\"v3SwapExactOut\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMin\",\"type\":\"uint256\"}],\"name\":\"wrapEth\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"x2y21155\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"name\":\"x2y2721\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ;
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
        #[doc = "Calls the contract's `cryptopunks` (0xeda3083f) function"]
        pub fn cryptopunks(
            &self,
            punk_id: ethers_core::types::U256,
            recipient: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 163, 8, 63], (punk_id, recipient, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `foundation` (0xa5c51c5c) function"]
        pub fn foundation(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 197, 28, 92], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `looksRare1155` (0x564c2f7e) function"]
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
                .method_hash([86, 76, 47, 126], (value, data, recipient, token, id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `looksRare721` (0x9289a6bd) function"]
        pub fn looks_rare_721(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 137, 166, 189], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nft20` (0x7bf8e837) function"]
        pub fn nft_20(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 248, 232, 55], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nftx` (0x9d49b90a) function"]
        pub fn nftx(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 73, 185, 10], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerCheck1155` (0x3e148b68) function"]
        pub fn owner_check_1155(
            &self,
            owner: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
            min_balance: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 20, 139, 104], (owner, token, id, min_balance))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerCheck721` (0xe0d26a5a) function"]
        pub fn owner_check_721(
            &self,
            owner: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 210, 106, 90], (owner, token, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payPortion` (0xd5092a81) function"]
        pub fn pay_portion(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            bips: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 9, 42, 129], (token, recipient, bips))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit2Permit` (0x9d8e5ab3) function"]
        pub fn permit_2_permit(
            &self,
            permit_single: PermitSingle,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 142, 90, 179], (permit_single, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit2PermitBatch` (0x5b9f25d3) function"]
        pub fn permit_2_permit_batch(
            &self,
            permit_batch: PermitBatch,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 159, 37, 211], (permit_batch, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit2TransferFrom` (0xf7223d79) function"]
        pub fn permit_2_transfer_from(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 34, 61, 121], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit2TransferFromBatch` (0x812a07a3) function"]
        pub fn permit_2_transfer_from_batch(
            &self,
            batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 42, 7, 163], batch_details)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seaport` (0x41cd3002) function"]
        pub fn seaport(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 205, 48, 2], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sudoswap` (0x9211495f) function"]
        pub fn sudoswap(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 17, 73, 95], (value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweep` (0x62c06767) function"]
        pub fn sweep(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 192, 103, 103], (token, recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepErc1155` (0xb930d445) function"]
        pub fn sweep_erc_1155(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            id: ethers_core::types::U256,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 48, 212, 69], (token, recipient, id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepErc721` (0x40e2cf43) function"]
        pub fn sweep_erc_721(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 226, 207, 67], (token, recipient, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xbeabacc8) function"]
        pub fn transfer(
            &self,
            token: ethers_core::types::Address,
            recipient: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 171, 172, 200], (token, recipient, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWeth` (0x521b6390) function"]
        pub fn unwrap_weth(
            &self,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 27, 99, 144], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `v2SwapExactIn` (0xdcd5db1f) function"]
        pub fn v_2_swap_exact_in(
            &self,
            recipient: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: ::std::vec::Vec<ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [220, 213, 219, 31],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `v2SwapExactOut` (0xe868b367) function"]
        pub fn v_2_swap_exact_out(
            &self,
            recipient: ethers_core::types::Address,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: ::std::vec::Vec<ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 104, 179, 103],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `v3SwapExactIn` (0x5a58ec2d) function"]
        pub fn v_3_swap_exact_in(
            &self,
            recipient: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 88, 236, 45],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `v3SwapExactOut` (0x14cd4424) function"]
        pub fn v_3_swap_exact_out(
            &self,
            recipient: ethers_core::types::Address,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 205, 68, 36],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapEth` (0xb3bc1a13) function"]
        pub fn wrap_eth(
            &self,
            recipient: ethers_core::types::Address,
            amount_min: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 188, 26, 19], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `x2y21155` (0x108ccf02) function"]
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
                .method_hash([16, 140, 207, 2], (value, data, recipient, token, id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `x2y2721` (0xc059e681) function"]
        pub fn x_2y_2721(
            &self,
            value: ethers_core::types::U256,
            data: ethers_core::types::Bytes,
            recipient: ethers_core::types::Address,
            token: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 89, 230, 129], (value, data, recipient, token, id))
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
    #[doc = "Container type for all input parameters for the `cryptopunks` function with signature `cryptopunks(uint256,address,uint256)` and selector `[237, 163, 8, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cryptopunks", abi = "cryptopunks(uint256,address,uint256)")]
    pub struct CryptopunksCall {
        pub punk_id: ethers_core::types::U256,
        pub recipient: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `foundation` function with signature `foundation(uint256,bytes,address,address,uint256)` and selector `[165, 197, 28, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "foundation", abi = "foundation(uint256,bytes,address,address,uint256)")]
    pub struct FoundationCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `looksRare1155` function with signature `looksRare1155(uint256,bytes,address,address,uint256,uint256)` and selector `[86, 76, 47, 126]`"]
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
        name = "looksRare1155",
        abi = "looksRare1155(uint256,bytes,address,address,uint256,uint256)"
    )]
    pub struct LooksRare1155Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `looksRare721` function with signature `looksRare721(uint256,bytes,address,address,uint256)` and selector `[146, 137, 166, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "looksRare721", abi = "looksRare721(uint256,bytes,address,address,uint256)")]
    pub struct LooksRare721Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `nft20` function with signature `nft20(uint256,bytes)` and selector `[123, 248, 232, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nft20", abi = "nft20(uint256,bytes)")]
    pub struct Nft20Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `nftx` function with signature `nftx(uint256,bytes)` and selector `[157, 73, 185, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nftx", abi = "nftx(uint256,bytes)")]
    pub struct NftxCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `ownerCheck1155` function with signature `ownerCheck1155(address,address,uint256,uint256)` and selector `[62, 20, 139, 104]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerCheck1155", abi = "ownerCheck1155(address,address,uint256,uint256)")]
    pub struct OwnerCheck1155Call {
        pub owner: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub min_balance: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `ownerCheck721` function with signature `ownerCheck721(address,address,uint256)` and selector `[224, 210, 106, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerCheck721", abi = "ownerCheck721(address,address,uint256)")]
    pub struct OwnerCheck721Call {
        pub owner: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `payPortion` function with signature `payPortion(address,address,uint256)` and selector `[213, 9, 42, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "payPortion", abi = "payPortion(address,address,uint256)")]
    pub struct PayPortionCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub bips: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `permit2Permit` function with signature `permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)` and selector `[157, 142, 90, 179]`"]
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
        name = "permit2Permit",
        abi = "permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)"
    )]
    pub struct Permit2PermitCall {
        pub permit_single: PermitSingle,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `permit2PermitBatch` function with signature `permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)` and selector `[91, 159, 37, 211]`"]
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
        name = "permit2PermitBatch",
        abi = "permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)"
    )]
    pub struct Permit2PermitBatchCall {
        pub permit_batch: PermitBatch,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `permit2TransferFrom` function with signature `permit2TransferFrom(address,address,uint160)` and selector `[247, 34, 61, 121]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "permit2TransferFrom", abi = "permit2TransferFrom(address,address,uint160)")]
    pub struct Permit2TransferFromCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `permit2TransferFromBatch` function with signature `permit2TransferFromBatch((address,address,uint160,address)[])` and selector `[129, 42, 7, 163]`"]
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
        name = "permit2TransferFromBatch",
        abi = "permit2TransferFromBatch((address,address,uint160,address)[])"
    )]
    pub struct Permit2TransferFromBatchCall {
        pub batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
    }
    #[doc = "Container type for all input parameters for the `seaport` function with signature `seaport(uint256,bytes)` and selector `[65, 205, 48, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "seaport", abi = "seaport(uint256,bytes)")]
    pub struct SeaportCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `sudoswap` function with signature `sudoswap(uint256,bytes)` and selector `[146, 17, 73, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sudoswap", abi = "sudoswap(uint256,bytes)")]
    pub struct SudoswapCall {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `sweep` function with signature `sweep(address,address,uint256)` and selector `[98, 192, 103, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sweep", abi = "sweep(address,address,uint256)")]
    pub struct SweepCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sweepErc1155` function with signature `sweepErc1155(address,address,uint256,uint256)` and selector `[185, 48, 212, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sweepErc1155", abi = "sweepErc1155(address,address,uint256,uint256)")]
    pub struct SweepErc1155Call {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sweepErc721` function with signature `sweepErc721(address,address,uint256)` and selector `[64, 226, 207, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sweepErc721", abi = "sweepErc721(address,address,uint256)")]
    pub struct SweepErc721Call {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,address,uint256)` and selector `[190, 171, 172, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,address,uint256)")]
    pub struct TransferCall {
        pub token: ethers_core::types::Address,
        pub recipient: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapWeth` function with signature `unwrapWeth(address,uint256)` and selector `[82, 27, 99, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unwrapWeth", abi = "unwrapWeth(address,uint256)")]
    pub struct UnwrapWethCall {
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `v2SwapExactIn` function with signature `v2SwapExactIn(address,uint256,uint256,address[],bool)` and selector `[220, 213, 219, 31]`"]
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
        name = "v2SwapExactIn",
        abi = "v2SwapExactIn(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactInCall {
        pub recipient: ethers_core::types::Address,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_min: ethers_core::types::U256,
        pub path: ::std::vec::Vec<ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `v2SwapExactOut` function with signature `v2SwapExactOut(address,uint256,uint256,address[],bool)` and selector `[232, 104, 179, 103]`"]
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
        name = "v2SwapExactOut",
        abi = "v2SwapExactOut(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactOutCall {
        pub recipient: ethers_core::types::Address,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_max: ethers_core::types::U256,
        pub path: ::std::vec::Vec<ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `v3SwapExactIn` function with signature `v3SwapExactIn(address,uint256,uint256,bytes,bool)` and selector `[90, 88, 236, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "v3SwapExactIn", abi = "v3SwapExactIn(address,uint256,uint256,bytes,bool)")]
    pub struct V3SwapExactInCall {
        pub recipient: ethers_core::types::Address,
        pub amount_in: ethers_core::types::U256,
        pub amount_out_min: ethers_core::types::U256,
        pub path: ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `v3SwapExactOut` function with signature `v3SwapExactOut(address,uint256,uint256,bytes,bool)` and selector `[20, 205, 68, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "v3SwapExactOut", abi = "v3SwapExactOut(address,uint256,uint256,bytes,bool)")]
    pub struct V3SwapExactOutCall {
        pub recipient: ethers_core::types::Address,
        pub amount_out: ethers_core::types::U256,
        pub amount_in_max: ethers_core::types::U256,
        pub path: ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    #[doc = "Container type for all input parameters for the `wrapEth` function with signature `wrapEth(address,uint256)` and selector `[179, 188, 26, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "wrapEth", abi = "wrapEth(address,uint256)")]
    pub struct WrapEthCall {
        pub recipient: ethers_core::types::Address,
        pub amount_min: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `x2y21155` function with signature `x2y21155(uint256,bytes,address,address,uint256,uint256)` and selector `[16, 140, 207, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "x2y21155", abi = "x2y21155(uint256,bytes,address,address,uint256,uint256)")]
    pub struct X2Y21155Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `x2y2721` function with signature `x2y2721(uint256,bytes,address,address,uint256)` and selector `[192, 89, 230, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers_contract :: EthCall,
        ethers_contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "x2y2721", abi = "x2y2721(uint256,bytes,address,address,uint256)")]
    pub struct X2Y2721Call {
        pub value: ethers_core::types::U256,
        pub data: ethers_core::types::Bytes,
        pub recipient: ethers_core::types::Address,
        pub token: ethers_core::types::Address,
        pub id: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers_contract :: EthAbiType)]
    pub enum IUniversalRouterCommandsCalls {
        Cryptopunks(CryptopunksCall),
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
        Sudoswap(SudoswapCall),
        Sweep(SweepCall),
        SweepErc1155(SweepErc1155Call),
        SweepErc721(SweepErc721Call),
        Transfer(TransferCall),
        UnwrapWeth(UnwrapWethCall),
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
                <CryptopunksCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Cryptopunks(decoded));
            }
            if let Ok(decoded) =
                <FoundationCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Foundation(decoded));
            }
            if let Ok(decoded) =
                <LooksRare1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::LooksRare1155(decoded));
            }
            if let Ok(decoded) =
                <LooksRare721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::LooksRare721(decoded));
            }
            if let Ok(decoded) = <Nft20Call as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Nft20(decoded));
            }
            if let Ok(decoded) = <NftxCall as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Nftx(decoded));
            }
            if let Ok(decoded) =
                <OwnerCheck1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::OwnerCheck1155(decoded));
            }
            if let Ok(decoded) =
                <OwnerCheck721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::OwnerCheck721(decoded));
            }
            if let Ok(decoded) =
                <PayPortionCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::PayPortion(decoded));
            }
            if let Ok(decoded) =
                <Permit2PermitCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2Permit(decoded));
            }
            if let Ok(decoded) =
                <Permit2PermitBatchCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2PermitBatch(decoded));
            }
            if let Ok(decoded) =
                <Permit2TransferFromCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <Permit2TransferFromBatchCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Permit2TransferFromBatch(decoded));
            }
            if let Ok(decoded) = <SeaportCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Seaport(decoded));
            }
            if let Ok(decoded) =
                <SudoswapCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Sudoswap(decoded));
            }
            if let Ok(decoded) = <SweepCall as ethers_core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniversalRouterCommandsCalls::Sweep(decoded));
            }
            if let Ok(decoded) =
                <SweepErc1155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::SweepErc1155(decoded));
            }
            if let Ok(decoded) =
                <SweepErc721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::SweepErc721(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWethCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::UnwrapWeth(decoded));
            }
            if let Ok(decoded) =
                <V2SwapExactInCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V2SwapExactIn(decoded));
            }
            if let Ok(decoded) =
                <V2SwapExactOutCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V2SwapExactOut(decoded));
            }
            if let Ok(decoded) =
                <V3SwapExactInCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V3SwapExactIn(decoded));
            }
            if let Ok(decoded) =
                <V3SwapExactOutCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::V3SwapExactOut(decoded));
            }
            if let Ok(decoded) = <WrapEthCall as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::WrapEth(decoded));
            }
            if let Ok(decoded) =
                <X2Y21155Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::X2Y21155(decoded));
            }
            if let Ok(decoded) = <X2Y2721Call as ethers_core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniversalRouterCommandsCalls::X2Y2721(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ethers_core::abi::AbiEncode for IUniversalRouterCommandsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniversalRouterCommandsCalls::Cryptopunks(element) => element.encode(),
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
                IUniversalRouterCommandsCalls::Sudoswap(element) => element.encode(),
                IUniversalRouterCommandsCalls::Sweep(element) => element.encode(),
                IUniversalRouterCommandsCalls::SweepErc1155(element) => element.encode(),
                IUniversalRouterCommandsCalls::SweepErc721(element) => element.encode(),
                IUniversalRouterCommandsCalls::Transfer(element) => element.encode(),
                IUniversalRouterCommandsCalls::UnwrapWeth(element) => element.encode(),
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
                IUniversalRouterCommandsCalls::Cryptopunks(element) => element.fmt(f),
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
                IUniversalRouterCommandsCalls::Sudoswap(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Sweep(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::SweepErc1155(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::SweepErc721(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::Transfer(element) => element.fmt(f),
                IUniversalRouterCommandsCalls::UnwrapWeth(element) => element.fmt(f),
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
    impl ::std::convert::From<CryptopunksCall> for IUniversalRouterCommandsCalls {
        fn from(var: CryptopunksCall) -> Self {
            IUniversalRouterCommandsCalls::Cryptopunks(var)
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
    impl ::std::convert::From<SudoswapCall> for IUniversalRouterCommandsCalls {
        fn from(var: SudoswapCall) -> Self {
            IUniversalRouterCommandsCalls::Sudoswap(var)
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
    impl ::std::convert::From<UnwrapWethCall> for IUniversalRouterCommandsCalls {
        fn from(var: UnwrapWethCall) -> Self {
            IUniversalRouterCommandsCalls::UnwrapWeth(var)
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
