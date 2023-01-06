/// Integers that are in the valid command range but are unassigned, and so don't have variants.
macro_rules! no_variants {
    () => {
        0x07 | 0x0e | 0x0f | 0x1e | 0x1f
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
    UnwrapEth,
    Permit2TransferFromBatch,
    // 0x0e
    // 0x0f

    // 0x10..0x18
    Seaport = 0x10,
    LooksRare721,
    Nftx,
    CryptoPunks,
    LooksRare1155,
    OwnerCheck721,
    OwnerCheck1155,
    SweepErc721,

    // 0x18..0x20
    X2y2721 = 0x18,
    SudoSwap,
    Nft20,
    X2y21155,
    Foundation,
    SweepErc1155,
    // 0x1e
    // 0x1f

    // (unused)  0x20..0x80
    // (invalid) 0x80..=0xff
    /// Placeholder for the currently unassigned commands.
    ///
    /// Is always equivalent to [`Command::V3SwapExactIn`] (0x00) when masked.
    Invalid = 0x80,
}

impl Command {
    /// The first 5 bits, the command value.
    pub const MASK: u8 = 0b00011111;
    /// The 5th and 6th bit, currently unused and will be ignored.
    pub const UNUSED_BITS: u8 = 0b01100000;
    /// The last bit, the `allow_revert` flag.
    pub const FLAG_ALLOW_REVERT: u8 = 0b10000000;

    /// ?
    pub const NFT_TYPE_MASK: u8 = 0b00010000;
    /// ?
    pub const SUB_IF_BRANCH_MASK: u8 = 0b00001000;

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
        const MAX: u8 = Command::MASK + 1;
        let command = match byte {
            // no variants yet
            no_variants!() => Self::Invalid,
            // outside of valid range
            MAX.. => Self::Invalid,
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
    fn command_encoding() {
        for byte in 0..=u8::MAX {
            let allow_revert = (byte >> 7) == 1;
            let (command, r_allow_revert) = Command::decode(byte);
            assert_eq!(r_allow_revert, allow_revert);
            const MAX: u8 = Command::MASK + 1;
            match byte {
                no_variants!() => assert_eq!(command, Command::Invalid),
                MAX.. => assert_eq!(command, Command::Invalid),
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
