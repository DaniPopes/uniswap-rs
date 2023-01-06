mod token;
pub use token::Erc20;

pub mod constants;
pub mod errors;
pub mod utils;

use ethers_core::types::U256;

/// A helper enum that wraps a [U256] for determining a swap's input / output amount.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Amount {
    /// Swap exactly {0} `TokenIn` for any amount of `TokenOut`.
    ExactIn(U256),
    /// Swap any amount of `TokenIn` for exactly {0} `TokenOut`.
    ExactOut(U256),
}

impl Amount {
    /// Swap exactly `amount` `TokenIn` for any amount of `TokenOut`.
    #[inline]
    pub fn exact_in<T: Into<U256>>(amount: T) -> Self {
        Self::ExactIn(amount.into())
    }

    /// Swap any amount of `TokenIn` for exactly `amount` `TokenOut`.
    #[inline]
    pub fn exact_out<T: Into<U256>>(amount: T) -> Self {
        Self::ExactOut(amount.into())
    }
}
