mod command;
pub use command::Command;

use crate::{
    contracts::bindings::{
        i_universal_router::{ExecuteCall, ExecuteWithCommandsAndInputsCall, IUniversalRouter},
        i_universal_router_commands::*,
    },
    utils::get_deadline,
};
use ethers_contract::builders::ContractCall;
use ethers_core::{
    abi::{self, Token, Tokenizable, Tokenize},
    types::{Address, Bytes, U256},
};
use ethers_providers::Middleware;
use std::{mem, sync::Arc};

#[cfg(feature = "addresses")]
use crate::protocol::ProtocolType;
#[cfg(feature = "addresses")]
use ethers_core::types::Chain;

macro_rules! add_command_fns {
    ($(
        $name:ident => pub fn $fn_name:ident ($($arg:ident : $ty:ty $(,)?)+) ;
    )+) => {$(
        #[doc = concat!("Append a [`", stringify!($name), "`][ref] command to the builder.\n\n")]
        #[doc = concat!("[ref]: https://docs.uniswap.org/contracts/universal-router/technical-reference#", stringify!($fn_name))]
        #[inline]
        pub fn $fn_name(&mut self, allow_revert: bool, $($arg: $ty),+) -> &mut Self {
            let __command = Command::$name;
            let __args = [$(
                Tokenizable::into_token($arg),
            )+];
            self.add_command(__command, allow_revert, &__args)
        }
    )+};
}

contract_struct! {
    /// Represents a UniversalRouter router.
    ///
    /// # Example
    ///
    /// ```
    /// # use uniswap_rs::prelude::{*, _ethers::*};
    /// # use uniswap_rs::{
    /// #     contracts::bindings::i_universal_router_commands::V2SwapExactInCall,
    /// # };
    /// # use std::sync::Arc;
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// // construct the router
    /// let address = Address::repeat_byte(0x11);
    /// let provider = Arc::new(Provider::<Http>::try_from("https://example.com")?);
    /// let mut router = UniversalRouter::new(provider, address);
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

        /// The raw command bytes.
        commands: Vec<u8>,

        /// The raw command inputs.
        inputs: Vec<Bytes>,
    }
}

impl<M: Middleware> UniversalRouter<M> {
    /// Creates a new instance using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IUniversalRouter::new(address, client);
        Self { contract, commands: Default::default(), inputs: Default::default() }
    }

    /// Creates a new instance by searching for the required addresses in the [addressbook].
    ///
    /// [addressbook]: crate::contracts::addresses
    #[cfg(feature = "addresses")]
    pub fn new_with_chain(client: Arc<M>, chain: Chain, protocol: ProtocolType) -> Option<Self> {
        protocol.try_addresses(chain).1.map(|address| Self::new(client, address))
    }

    /// Reserves capacity for at least additional more elements to be inserted.
    ///
    /// After calling `reserve`, capacity will be greater than or equal to `self.len() +
    /// additional`. Does nothing if capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) -> &mut Self {
        self.commands.reserve(additional);
        self.inputs.reserve(additional);
        self
    }

    /// Clears the internal buffers, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the buffers.
    pub fn clear(&mut self) -> &mut Self {
        self.commands.clear();
        self.inputs.clear();
        self
    }

    /// Add a command binding, and whether it is allowed to revert to the call.
    ///
    /// # Example
    ///
    /// ```
    /// # use uniswap_rs::prelude::{*, _ethers::*};
    /// # use ethers_core::abi::Token;
    /// # let mut router = UniversalRouter::new(Provider::<Http>::try_from("http://example.com").unwrap().into(), Address::zero());
    /// let address = Address::zero();
    /// let amount = U256::zero();
    /// router.add_command(Command::WrapEth, false, &[Token::Address(address), Token::Uint(amount)]);
    /// // equivalent to:
    /// router.wrap_eth(false, address, amount);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn add_command_from_bindings(
        &mut self,
        command: IUniversalRouterCommandsCalls,
        allow_revert: bool,
    ) -> &mut Self {
        let command_type = Command::from(&command);
        let tokens = command.into_tokens();
        self.add_command(command_type, allow_revert, &tokens)
    }

    /// Add a command, its arguments, and whether it is allowed to revert to the call.
    ///
    /// **Important**: this method is not type-safe. It is recommended to use the `command_name`
    /// methods instead. See [this type's documentation][UniversalRouter] on how to do this.
    ///
    /// # Example
    ///
    /// ```
    /// # use uniswap_rs::prelude::{*, _ethers::*};
    /// # use ethers_core::abi::Token;
    /// # let mut router = UniversalRouter::new(Provider::<Http>::try_from("http://example.com").unwrap().into(), Address::zero());
    /// let address = Token::Address(Address::zero());
    /// let amount = Token::Uint(U256::zero());
    /// router.add_command(Command::WrapEth, false, &[address, amount]);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn add_command(
        &mut self,
        command: Command,
        allow_revert: bool,
        args: &[Token],
    ) -> &mut Self {
        let command_byte = command.encode(allow_revert);
        let input = abi::encode(args).into();
        self.add_command_raw(command_byte, input)
    }

    /// Add a command, its input bytes, and whether it is allowed to revert to the call.
    ///
    /// **Important**: this method is not type-safe. It is recommended to use the `command_name`
    /// methods instead. See [this type's documentation][UniversalRouter] on how to do this.
    pub fn add_command_raw(&mut self, command_byte: u8, input: Bytes) -> &mut Self {
        self.commands.push(command_byte);
        self.inputs.push(input);
        self
    }

    /// Consumes the internal buffers to build into [`ExecuteWithCommandsAndInputsCall`].
    pub fn build(&mut self, deadline: U256) -> ExecuteWithCommandsAndInputsCall {
        let commands = mem::take(&mut self.commands);
        let inputs = mem::take(&mut self.inputs);
        ExecuteWithCommandsAndInputsCall { commands: commands.into(), inputs, deadline }
    }

    /// Consumes the internal buffers to build into [`ExecuteCall`].
    pub fn build_no_deadline(&mut self) -> ExecuteCall {
        let commands = mem::take(&mut self.commands);
        let inputs = mem::take(&mut self.inputs);
        ExecuteCall { commands: commands.into(), inputs }
    }

    /// Consumes the internal buffers to create a call to the [`router`][IUniversalRouter]'s
    /// `execute` function.
    ///
    /// `deadline` is measured in seconds from when this method is called.
    ///
    /// # Example
    ///
    /// ```
    /// # use uniswap_rs::prelude::{*, _ethers::*};
    /// # use ethers_core::abi::Token;
    /// # let mut router = UniversalRouter::new(Provider::<Http>::try_from("http://example.com").unwrap().into(), Address::zero());
    /// let address = Token::Address(Address::zero());
    /// let amount = Token::Uint(U256::zero());
    /// let deadline = Some(300);
    /// router.add_command(Command::WrapEth, false, &[address, amount]);
    /// let call = router.call(deadline);
    /// // send or simulate the call ...
    /// // call.value(amount).send().await?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn call(&mut self, deadline: Option<u64>) -> ContractCall<M, ()> {
        let commands: Bytes = mem::take(&mut self.commands).into();
        let inputs = mem::take(&mut self.inputs);
        match deadline {
            Some(deadline) => {
                let deadline = get_deadline(deadline);
                self.contract.execute_with_commands_and_inputs(commands, inputs, deadline)
            }
            None => self.contract.execute(commands, inputs),
        }
    }
}

// implement commands in a new block so as to not "pollute" docs
impl<M: Middleware> UniversalRouter<M> {
    add_command_fns! {
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
        UnwrapWeth => pub fn unwrap_weth(
            recipient: Address,
            amount_min: U256
        );
        Permit2TransferFromBatch => pub fn permit2_transfer_from_batch(
            batch_details: Vec<AllowanceTransferDetails>
        );
        Seaport => pub fn seaport(
            value: U256,
            data: Bytes
        );
        LooksRare721 => pub fn looks_rare_721(
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
        Cryptopunks => pub fn cryptopunks(
            punk_id: U256,
            recipient: Address,
            value: U256
        );
        LooksRare1155 => pub fn looks_rare_1155(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256,
            amount: U256
        );
        OwnerCheck721 => pub fn owner_check_721(
            owner: Address,
            token: Address,
            id: U256
        );
        OwnerCheck1155 => pub fn owner_check_1155(
            owner: Address,
            token: Address,
            id: U256,
            min_balance: U256
        );
        SweepErc721 => pub fn sweep_erc721(
            token: Address,
            recipient: Address,
            id: U256
        );
        X2Y2721 => pub fn x2y2_721(
            value: U256,
            data: Bytes,
            recipient: Address,
            token: Address,
            id: U256
        );
        Sudoswap => pub fn sudoswap(
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
            id: U256,
            amount: U256
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::bindings::i_universal_router_commands::SweepCall;
    use ethers_core::{
        abi::Tokenize,
        types::{Address, U256},
    };
    use ethers_providers::{MockProvider, Provider};

    #[test]
    fn test_builder() {
        let token = Address::from_low_u64_be(1);
        let recipient = Address::from_low_u64_be(2);
        let amount_min = U256::from(3);
        let command = SweepCall { token, recipient, amount_min };
        let allow_revert = false;

        let tokens = command.clone().into_tokens();
        let encoded = ethers_core::abi::encode(&tokens);

        let e_commands = Bytes::from(vec![Command::Sweep.encode(allow_revert)]);
        let e_inputs = vec![Bytes::from(encoded)];

        let provider = Provider::new(MockProvider::new()).into();
        let mut router = UniversalRouter::new(provider, Address::zero());

        router.add_command(Command::Sweep, allow_revert, &tokens);
        assert_eq!(router.commands, e_commands);
        assert_eq!(router.inputs, e_inputs);

        router.clear();

        router.add_command_from_bindings(command.into(), allow_revert);
        assert_eq!(router.commands, e_commands);
        assert_eq!(router.inputs, e_inputs);
    }
}
