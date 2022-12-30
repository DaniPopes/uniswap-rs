use super::Command;
use crate::{
    bindings::{
        i_universal_router::{ExecuteCall, ExecuteWithCommandsAndInputsCall, IUniversalRouter},
        i_universal_router_commands::IUniversalRouterCommandsCalls,
    },
    utils::get_deadline,
};
use ethers::{
    abi::Tokenize, contract::builders::ContractCall, providers::Middleware, types::Bytes,
};

/// Builds a call to the UniversalRouter's `execute` function.
///
/// # Example
///
/// ``` no_run
/// use ethers::prelude::*;
/// use uniswap_rs::{
///     bindings::{i_universal_router_commands::V2SwapExactInCall, i_universal_router::IUniversalRouter},
///     v3::Builder,
/// };
/// use std::{convert::TryFrom, sync::Arc};
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// // construct the router
/// let router = Address::random();
/// let provider = Arc::new(Provider::<Http>::try_from("https://example.com")?);
/// let router = IUniversalRouter::new(router, provider);
///
/// // construct the commands
/// let token = Address::repeat_byte(0x11);
/// let recipient = Address::repeat_byte(0x24);
/// let value = U256::from(1_000_000_000_u64);
/// let swap = V2SwapExactInCall {
///     recipient,
///     amount_in: value,
///     amount_out_min: U256::zero(),
///     path: vec![Address::zero(), token],
///     payer_is_user: true,
/// }.into();
///
/// // construct the call
/// let call = Builder::new().command(swap, false).call(&router, None).value(value);
///
/// // send the call
/// let pending = call.send().await?;
/// let _receipt = pending.await?.expect("dropped");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Builder {
    commands: Vec<u8>,
    inputs: Vec<Bytes>,
}

impl Builder {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Constructs a new, empty builder with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { commands: Vec::with_capacity(capacity), inputs: Vec::with_capacity(capacity) }
    }

    /// Reserves capacity for at least additional more elements to be inserted.
    ///
    /// After calling `reserve`, capacity will be greater than or equal to `self.len() +
    /// additional`. Does nothing if capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) {
        self.commands.reserve(additional);
        self.inputs.reserve(additional);
    }

    /// Consumes `self` to build into [`ExecuteWithCommandsAndInputsCall`].
    pub fn build(self, deadline: Option<u64>) -> ExecuteWithCommandsAndInputsCall {
        let Self { commands, inputs } = self;
        ExecuteWithCommandsAndInputsCall {
            commands: commands.into(),
            inputs,
            deadline: get_deadline(deadline),
        }
    }

    /// Consumes `self` to build into [`ExecuteCall`].
    pub fn build_no_deadline(self) -> ExecuteCall {
        let Self { commands, inputs, .. } = self;
        ExecuteCall { commands: commands.into(), inputs }
    }

    /// Consumes `self` to create a call to the [`router`][IUniversalRouter]'s `execute` function.
    pub fn call<M: Middleware>(
        self,
        router: &IUniversalRouter<M>,
        deadline: Option<u64>,
    ) -> ContractCall<M, ()> {
        match deadline {
            Some(deadline) => {
                let ExecuteWithCommandsAndInputsCall { commands, inputs, deadline } =
                    self.build(Some(deadline));
                router.execute_with_commands_and_inputs(commands, inputs, deadline)
            }
            None => {
                let ExecuteCall { commands, inputs } = self.build_no_deadline();
                router.execute(commands, inputs)
            }
        }
    }

    /// Add a command and whether it is allowed to revert to the call.
    pub fn command(self, command: IUniversalRouterCommandsCalls, allow_revert: bool) -> Self {
        use IUniversalRouterCommandsCalls::*;
        let command_type = match &command {
            // 0x00..0x08
            V3SwapExactIn(_) => Command::V3SwapExactIn,
            V3SwapExactOut(_) => Command::V3SwapExactOut,
            Permit2TransferFrom(_) => Command::Permit2TransferFrom,
            Permit2PermitBatch(_) => Command::Permit2PermitBatch,
            Sweep(_) => Command::Sweep,
            Transfer(_) => Command::Transfer,
            PayPortion(_) => Command::PayPortion,
            // 0x07

            // 0x08..0x10
            V2SwapExactIn(_) => Command::V2SwapExactIn,
            V2SwapExactOut(_) => Command::V2SwapExactOut,
            Permit2Permit(_) => Command::Permit2Permit,
            WrapEth(_) => Command::WrapEth,
            UnwrapEth(_) => Command::UnwrapEth,
            Permit2TransferFromBatch(_) => Command::Permit2TransferFromBatch,
            // 0x0e
            // 0x0f

            // 0x10..0x18
            Seaport(_) => Command::Seaport,
            LooksRare721(_) => Command::LooksRare721,
            Nftx(_) => Command::Nftx,
            CryptoPunks(_) => Command::CryptoPunks,
            LooksRare1155(_) => Command::LooksRare1155,
            OwnerCheck721(_) => Command::OwnerCheck721,
            OwnerCheck1155(_) => Command::OwnerCheck1155,
            SweepErc721(_) => Command::SweepErc721,

            // 0x18..0x20
            X2Y2721(_) => Command::X2y2721,
            SudoSwap(_) => Command::SudoSwap,
            Nft20(_) => Command::Nft20,
            X2Y21155(_) => Command::X2y21155,
            Foundation(_) => Command::Foundation,
            SweepErc1155(_) => Command::SweepErc1155,
            // 0x1e
            // 0x1f
        };
        let input = ethers::abi::encode(&command.into_tokens()).into();
        self.command_raw(command_type, allow_revert, input)
    }

    /// Add a command, its input bytes, and whether it is allowed to revert to the call.
    ///
    /// Note: no checks are done on the input.
    pub fn command_raw(mut self, command: Command, allow_revert: bool, input: Bytes) -> Self {
        self.commands.push(command.encode(allow_revert));
        self.inputs.push(input);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::i_universal_router_commands::SweepCall;
    use ethers::types::{Address, U256};

    #[test]
    fn builder() {
        let token = Address::from_low_u64_be(1);
        let recipient = Address::from_low_u64_be(2);
        let amount_min = U256::from(3);
        let command = SweepCall { token, recipient, amount_min };
        let allow_revert = false;

        let encoded = ethers::abi::encode(&command.clone().into_tokens());

        let r = Builder::new().command(command.into(), allow_revert).build(None);
        assert_eq!(r.commands, Bytes::from(vec![Command::Sweep.encode(allow_revert)]));
        assert_eq!(r.inputs, vec![Bytes::from(encoded)]);
    }
}
