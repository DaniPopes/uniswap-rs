//! Errors

use ethers_contract::{ContractError, MulticallError};
use ethers_core::abi::InvalidOutputType;
use ethers_providers::{Middleware, ProviderError};
use thiserror::Error as ThisError;

/// Type alias for Result<T, E = Error>
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error thrown by Uniswap.
#[derive(Debug, ThisError)]
#[rustfmt::skip]
pub enum Error {
    /* ----------------------------------------- Generic ---------------------------------------- */

    /// Thrown by interacted smart contracts.
    #[error("{0}")]
    ContractError(String),

    /// Thrown when interacting with [Multicall][ethers::contract::Multicall].
    #[error("{0}")]
    MulticallError(String),

    /// Thrown when a provider call fails.
    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    /* ------------------------------------------- Dex ------------------------------------------ */

    /// Thrown when the provided slippage is invalid.
    #[error("Slippage must be in range: 0.0..=100.0")]
    InvalidSlippage,

    /// Thrown when the start and finish token are the same.
    #[error("Cannot swap a token into itself")]
    SwapToSelf,

    /// Thrown when trying to create a WETH deposit or withdrawal and WETH has not been set.
    #[error("WETH has yet to be set")]
    WethNotSet,

    /* ----------------------------------------- Library ---------------------------------------- */

    /// Thrown when providing identical addresses as parameters.
    #[error("Sorting identical addresses")]
    IdenticalAddresses,

    /// Thrown when providing an input amount equal to zero.
    #[error("Input amount is zero")]
    InsufficientInputAmount,

    /// Thrown when providing an output amount equal to zero.
    #[error("Output amount is zero")]
    InsufficientOutputAmount,

    /// Thrown when providing a liquidity amount equal to zero.
    #[error("Liquidity is zero")]
    InsufficientLiquidity,

    /// Thrown when the provided path is empty or contains only one address.
    #[error("Path length must be greater than or equal to 2")]
    InvalidPath,

    /// Thrown when the factory provided returns none for pair_code_hash
    #[error("Custom protocol is missing pair_code_hash")]
    NoPairCodeHash,
}

// Workaround for removing generic type in [Error].
impl<M: Middleware> From<ContractError<M>> for Error {
    fn from(value: ContractError<M>) -> Self {
        Self::ContractError(value.to_string())
    }
}

impl From<InvalidOutputType> for Error {
    fn from(value: InvalidOutputType) -> Self {
        Self::ContractError(value.to_string())
    }
}

impl<M: Middleware> From<MulticallError<M>> for Error {
    fn from(value: MulticallError<M>) -> Self {
        Self::MulticallError(value.to_string())
    }
}
