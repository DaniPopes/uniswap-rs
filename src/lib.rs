//! # uniswap-rs
//!
//! Simple library for interacting with Uniswap v2 and v3 smart contracts.

#![warn(missing_docs, unreachable_pub)]
#![deny(broken_intra_doc_links)]

mod constants;
mod dex;
mod factory;
mod library;
mod pair;
mod utils;

/// Contains the bindings for related smart contracts. Generated programmatically using [`Abigen`].
///
/// [`Abigen`]: ethers::contract::Abigen
#[allow(missing_docs, unreachable_pub)]
pub mod bindings;

/// Contains all related smart contract addresses. Modified from [`addressbook`].
///
/// [`addressbook`]: ethers::addressbook
pub mod contracts;

pub use constants::*;
pub use dex::*;
pub use factory::*;
pub use library::*;
pub use pair::*;
