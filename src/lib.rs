//! # uniswap-rs
//!
//! Simple library for interacting with Uniswap v2 and v3 smart contracts.

#![warn(missing_docs, unreachable_pub)]
#![deny(broken_intra_doc_links)]

mod common;
mod dex;
mod protocol;

pub mod v2;
pub mod v3;

/// Contains the bindings for related smart contracts. Generated programmatically using [`Abigen`].
///
/// [`Abigen`]: ethers::contract::Abigen
#[allow(missing_docs, unreachable_pub)]
pub mod bindings;

pub mod contracts;

pub use common::*;
pub use constants::*;
pub use dex::*;
pub use protocol::*;
