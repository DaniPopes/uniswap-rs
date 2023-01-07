use crate::contracts::bindings::i_universal_router_commands::IUniversalRouterCommandsCalls;

const MAX_COMMAND: u8 = Command::MASK + 1;

/// Integers that are in the valid command range but don't have variants.
macro_rules! no_variants {
    () => {
        0x07 | 0x0e | 0x0f | 0x1e..=Command::MASK
    };
}

/// Integers that are outside the valid command range.
macro_rules! invalid {
    () => {
        MAX_COMMAND..
    };
}

/// A [Universal Router command](https://docs.uniswap.org/contracts/universal-router/technical-reference#command).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Command {
    // 0x00..0x08
    #[default]
    V3SwapExactIn = 0x00,
    V3SwapExactOut,
    Permit2TransferFrom,
    Permit2PermitBatch,
    Sweep,
    Transfer,
    PayPortion,
    // 0x07

    // 0x08..0x10
    V2SwapExactIn = 0x08,
    V2SwapExactOut,
    Permit2Permit,
    WrapEth,
    UnwrapWeth,
    Permit2TransferFromBatch,
    // 0x0e
    // 0x0f

    // 0x10..0x18
    Seaport = 0x10,
    LooksRare721,
    Nftx,
    Cryptopunks,
    LooksRare1155,
    OwnerCheck721,
    OwnerCheck1155,
    SweepErc721,

    // 0x18..0x20
    X2Y2721 = 0x18,
    Sudoswap,
    Nft20,
    X2Y21155,
    Foundation,
    SweepErc1155,
    // 0x1e
    // 0x1f

    // (unassigned) 0x20.. 0x40
    // (unused)     0x40.. 0x80
    // (invalid)    0x80..=0xff
    /// Placeholder for the currently unassigned commands.
    ///
    /// Is always equivalent to [`Command::V3SwapExactIn`] (0x00) when masked.
    Invalid = 0x80,
}

impl From<&IUniversalRouterCommandsCalls> for Command {
    fn from(value: &IUniversalRouterCommandsCalls) -> Self {
        use IUniversalRouterCommandsCalls::*;

        match value {
            // 0x00..0x08
            V3SwapExactIn(_) => Self::V3SwapExactIn,
            V3SwapExactOut(_) => Self::V3SwapExactOut,
            Permit2TransferFrom(_) => Self::Permit2TransferFrom,
            Permit2PermitBatch(_) => Self::Permit2PermitBatch,
            Sweep(_) => Self::Sweep,
            Transfer(_) => Self::Transfer,
            PayPortion(_) => Self::PayPortion,
            // 0x07

            // 0x08..0x10
            V2SwapExactIn(_) => Self::V2SwapExactIn,
            V2SwapExactOut(_) => Self::V2SwapExactOut,
            Permit2Permit(_) => Self::Permit2Permit,
            WrapEth(_) => Self::WrapEth,
            UnwrapWeth(_) => Self::UnwrapWeth,
            Permit2TransferFromBatch(_) => Self::Permit2TransferFromBatch,
            // 0x0e
            // 0x0f

            // 0x10..0x18
            Seaport(_) => Self::Seaport,
            LooksRare721(_) => Self::LooksRare721,
            Nftx(_) => Self::Nftx,
            Cryptopunks(_) => Self::Cryptopunks,
            LooksRare1155(_) => Self::LooksRare1155,
            OwnerCheck721(_) => Self::OwnerCheck721,
            OwnerCheck1155(_) => Self::OwnerCheck1155,
            SweepErc721(_) => Self::SweepErc721,

            // 0x18..0x20
            X2Y2721(_) => Self::X2Y2721,
            Sudoswap(_) => Self::Sudoswap,
            Nft20(_) => Self::Nft20,
            X2Y21155(_) => Self::X2Y21155,
            Foundation(_) => Self::Foundation,
            SweepErc1155(_) => Self::SweepErc1155,
            // 0x1e
            // 0x1f
        }
    }
}

impl From<IUniversalRouterCommandsCalls> for Command {
    fn from(value: IUniversalRouterCommandsCalls) -> Self {
        From::from(&value)
    }
}

impl Command {
    /// The first 6 bits; the command value.
    pub const MASK: u8 = 0b00111111;
    /// The 6th bit; currently unused and will be ignored.
    pub const UNUSED_BITS: u8 = 0b01000000;
    /// The last bit; the `allow_revert` flag.
    pub const FLAG_ALLOW_REVERT: u8 = 0b10000000;

    /// Encodes the command to a single byte.
    ///
    /// Note: [`Command::Invalid`] will return the same value as [`Command::V3SwapExactIn`] (0x00)
    /// since it occupies an invalid bit that gets ignored.
    pub fn encode(self, allow_revert: bool) -> u8 {
        (self as u8 & Self::MASK) | ((allow_revert as u8) << 7)
    }

    /// Decodes the command from a single byte.
    ///
    /// Returns [Command::Invalid] if the command integer is a currently un-implemented or invalid
    /// command integer.
    pub fn decode(byte: u8) -> (Self, bool) {
        // last bit
        let allow_revert = (byte >> 7) == 1;
        let command = match byte {
            // no variants yet
            no_variants!() => Self::Invalid,
            // outside of valid range
            invalid!() => Self::Invalid,
            // SAFETY: All invalid values are covered in the match arms above.
            _ => unsafe { std::mem::transmute(byte) },
        };
        (command, allow_revert)
    }

    /// Returns whether this command is valid.
    #[allow(clippy::match_like_matches_macro)]
    pub const fn is_valid(&self) -> bool {
        match self {
            Self::Invalid => false,
            _ => true,
        }
    }

    /// Returns whether this command is invalid.
    #[allow(clippy::match_like_matches_macro)]
    pub const fn is_invalid(&self) -> bool {
        match self {
            Self::Invalid => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_encoding() {
        for byte in 0..=u8::MAX {
            let allow_revert = (byte >> 7) == 1;
            let (command, r_allow_revert) = Command::decode(byte);
            assert_eq!(r_allow_revert, allow_revert);
            match byte {
                no_variants!() => assert_eq!(command, Command::Invalid),
                invalid!() => assert_eq!(command, Command::Invalid),
                _ => {
                    assert_ne!(command, Command::Invalid);
                    assert_eq!(command.encode(allow_revert), byte & !Command::UNUSED_BITS);
                    // catch invalid references by formatting the command
                    let _s = format!("0x{byte:02x} => {command:?}");
                }
            };
        }
    }
}
