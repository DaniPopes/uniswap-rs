//! # uniswap-rs
//!
//! Unofficial Rust SDK library for Uniswap smart contracts.

#![allow(clippy::too_many_arguments)]
#![warn(missing_docs, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
#[doc(hidden)]
mod macros;

mod common;
mod dex;
mod protocol;

pub mod contracts;
pub mod v2;
pub mod v3;

pub use common::*;
pub use dex::*;
pub use protocol::*;

/// Easy imports of frequently used type definitions and traits.
#[doc(hidden)]
pub mod prelude {
    pub use super::{
        constants::*,
        v2::{
            Factory as V2Factory, Library as V2Library, Pair as V2Pair, Protocol as V2Protocol,
            Router as V2Router,
        },
        v3::{Command, Factory as V3Factory, FeeAmount, Pool as V3Pool, UniversalRouter},
    };

    #[cfg(feature = "addresses")]
    pub use super::contracts::addresses::{address, contract, try_address, try_contract};

    // convenience re-export of all ethers_* as one module.
    #[doc(hidden)]
    pub mod ethers {
        pub use ethers_contract::{self as contract, builders::*, *};
        pub use ethers_core::{
            self as core, abi,
            types::{self, *},
            *,
        };
        pub use ethers_providers::{self as providers, *};
    }
}
