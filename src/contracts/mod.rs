//! Uniswap smart contracts' bindings and addresses.

#[cfg(feature = "addresses")]
pub mod addresses;

// override abigen docs

/// Type-safe bindings to the Uniswap smart contracts.
///
/// Automatically generated using [`Abigen`][ethers_contract::Abigen].
#[allow(missing_docs)]
pub mod bindings {
    #[path = "../bindings/mod.rs"]
    #[doc(hidden)]
    mod _bindings;

    pub use _bindings::{
        i_swap_router, i_uniswap_v2_factory, i_uniswap_v2_pair, i_uniswap_v2_router_02,
        i_uniswap_v3_factory, i_uniswap_v3_pool, i_universal_router, ierc20, iweth,
    };

    // should not be used directly as it's not a valid contract
    #[doc(hidden)]
    pub use _bindings::i_universal_router_commands;
}
