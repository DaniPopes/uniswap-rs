//! # uniswap-rs
//!
//! Unofficial Rust SDK library for Uniswap smart contracts.

#![allow(clippy::too_many_arguments)]
#![warn(missing_docs, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]

mod common;
mod dex;
mod protocol;

#[allow(missing_docs)]
pub mod bindings;
pub mod contracts;
pub mod v2;
pub mod v3;

pub use common::*;
pub use dex::*;
pub use protocol::*;
