//! Uniswap smart contracts' bindings and addresses.

#[cfg(feature = "addresses")]
pub mod addresses;

// override docs
/// Type-safe bindings to the Uniswap smart contracts.
///
/// Automatically generated using [`Abigen`][ethers::contract::Abigen].
#[allow(missing_docs)]
pub mod bindings {
    #[path = "../bindings/mod.rs"]
    #[doc(hidden)]
    mod _bindings;

    pub use _bindings::*;
}
