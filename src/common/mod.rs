mod token;
pub use token::Erc20;

pub mod constants;
pub mod errors;
pub mod utils;

use ethers_core::types::U256;

/// A helper enum that wraps a [U256] for determining a swap's input / output amount.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Amount {
    /// Swap exactly {0} Token1 for any amount of Token2.
    ExactIn(U256),
    /// Swap any amount of Token1 for exactly {0} Token2.
    ExactOut(U256),
}
