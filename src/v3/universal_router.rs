use super::Builder;
use crate::contracts::bindings::{
    i_universal_router::IUniversalRouter, i_universal_router_commands::*,
};
use ethers_contract::builders::ContractCall;
use ethers_core::types::{Address, Bytes, U256};
use ethers_providers::Middleware;
use std::sync::Arc;

#[cfg(feature = "addresses")]
use crate::protocol::ProtocolType;
#[cfg(feature = "addresses")]
use ethers_core::types::Chain;

macro_rules! cmds {
    ($(
        $name:ident => pub fn $fn_name:ident ($($arg:ident : $ty:ty $(,)?)+) ;
    )+) => {$(
        #[doc = concat!("Append a [`", stringify!($name), "`][ref] command to the builder.\n\n")]
        #[doc = concat!("[ref]: https://docs.uniswap.org/contracts/universal-router/technical-reference#", stringify!($fn_name))]
        #[inline]
        pub fn $fn_name(mut self, allow_revert: bool, $($arg: $ty),+) -> Self {
            // concat_idents! workaround to instantiate a struct with braces syntax
            // #[cfg(feature = "nightly")]
            // type __Command = concat_idents!($name, Call);

            // #[cfg(not(feature = "nightly"))]
            type __Command = paste::paste!([<$name Call>]);

            let command = __Command { $($arg),+ };
            let command = IUniversalRouterCommandsCalls::$name(command);
            self.builder = self.builder.command(command, allow_revert);
            self
        }
    )+};
}

contract_struct! {
    /// Represents a UniversalRouter router.
    ///
    /// # Example
    ///
    /// ```
    /// # use ethers_core::types::*;
    /// # use ethers_contract::{builders::*, *};
    /// # use ethers_providers::*;
    /// # use uniswap_rs::{
    /// #     bindings::i_universal_router_commands::V2SwapExactInCall,
    /// #     constants::NATIVE_ADDRESS,
    /// #     v3::UniversalRouter,
    /// # };
    /// # use std::sync::Arc;
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// // construct the router
    /// let address = Address::repeat_byte(0x11);
    /// let provider = Arc::new(Provider::<Http>::try_from("https://example.com")?);
    /// let router = UniversalRouter::new(provider, address);
    ///
    /// // construct the call
    /// let token = Address::repeat_byte(0x22);
    /// let recipient = Address::repeat_byte(0x33);
    /// let value = U256::from(5) * U256::exp10(16); // 0.05 ETH
    /// let call = router
    ///     .v2_swap_exact_in(false, recipient, value, U256::zero(), vec![NATIVE_ADDRESS, token], true)
    /// //  .command_name(allow_revert, ...params) ...
    ///     .call(None)
    ///     .value(value);
    ///
    /// // send the call
    /// let pending = call.send().await?;
    /// let receipt = pending.await?.expect("dropped");
    /// println!("Tx successfully confirmed: {receipt:#?}");
    /// # Ok(()) }
    /// ```
    pub struct UniversalRouter<M> {
        /// The router contract.
        contract: IUniversalRouter<M>,

        /// The command builder.
        builder: Builder,
    }
}

impl<M: Middleware> UniversalRouter<M> {
    /// Creates a new instance using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IUniversalRouter::new(address, client);
        Self { contract, builder: Builder::with_capacity(3) }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        protocol.try_addresses(chain).1.map(|address| Self::new(client, address))
    }

    /// Consumes the builder to create a call to the `execute` function.
    pub fn call(&mut self, deadline: Option<u64>) -> ContractCall<M, ()> {
        let builder = std::mem::take(&mut self.builder);
        builder.call(&self.contract, deadline)
    }

    cmds! {
        V3SwapExactIn => pub fn v3_swap_exact_in(
            recipient: Address,
            amount_in: U256,
            amount_out_min: U256,
            path: Bytes,
            payer_is_user: bool
        );
        V3SwapExactOut => pub fn v3_swap_exact_out(
            recipient: Address,
            amount_out: U256,
            amount_in_max: U256,
            path: Bytes,
            payer_is_user: bool
        );
        Permit2TransferFrom => pub fn permit2_transfer_from(
            token: Address,
            recipient: Address,
            amount: U256
        );
        Permit2PermitBatch => pub fn permit2_permit_batch(
            permit_batch: PermitBatch,
            data: Bytes
        );
        Sweep => pub fn sweep(
            token: Address,
            recipient: Address,
            amount_min: U256
        );
        Transfer => pub fn transfer(
            token: Address,
            recipient: Address,
            value: U256
        );
        PayPortion => pub fn pay_portion(
            token: Address,
            recipient: Address,
            bips: U256
        );

        // 0x08..0x10
        V2SwapExactIn => pub fn v2_swap_exact_in(
            recipient: Address,
            amount_in: U256,
            amount_out_min: U256,
            path: Vec<Address>,
            payer_is_user: bool
        );
        V2SwapExactOut => pub fn v2_swap_exact_out(
            recipient: Address,
            amount_out: U256,
            amount_in_max: U256,
            path: Vec<Address>,
            payer_is_user: bool
        );
        Permit2Permit => pub fn permit2_permit(
            permit_single: PermitSingle,
            data: Bytes
        );
        WrapEth => pub fn wrap_eth(
            recipient: Address,
            amount_min: U256
        );
        UnwrapEth => pub fn unwrap_eth(
            recipient: Address,
            amount_min: U256
        );
        Permit2TransferFromBatch => pub fn permit2_transfer_from_batch(
            batch_details: Vec<AllowanceTransferDetails>
        );
        // 0x0e
        // 0x0f

        // 0x10..0x18
        Seaport => pub fn seaport(
            value: U256,
            data: Bytes
        );
        LooksRare721 => pub fn looks_rare721(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256
        );
        Nftx => pub fn nftx(
            value: U256,
            data: Bytes
        );
        CryptoPunks => pub fn crypto_punks(
            punk_id: U256,
            recipient: Address,
            value: U256
        );
        LooksRare1155 => pub fn looks_rare1155(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256,
            amount: U256
        );
        OwnerCheck721 => pub fn owner_check721(
            owner: Address,
            token: Address,
            id: U256
        );
        OwnerCheck1155 => pub fn owner_check1155(
            owner: Address
            token: Address,
            id: U256
        );
        SweepErc721 => pub fn sweep_erc721(
            token: Address,
            recipient: Address,
            id: U256
        );

        // 0x18..0x20
        X2Y2721 => pub fn x2y2_721(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256
        );
        SudoSwap => pub fn sudo_swap(
            value: U256,
            data: Bytes
        );
        Nft20 => pub fn nft20(
            value: U256,
            data: Bytes
        );
        X2Y21155 => pub fn x2y2_1155(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256,
            amount: U256
        );
        Foundation => pub fn foundation(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256
        );
        SweepErc1155 => pub fn sweep_erc1155(
            token: Address,
            recipient: Address,
            id: U256
        );
    }
}
