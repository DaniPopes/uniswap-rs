//! Utils

use crate::constants::NATIVE_ADDRESS;
use ethers::prelude::*;
use std::time::{Duration, SystemTime};

/// Returns the [Duration] since the UNIX epoch.
pub fn now() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}

/// Returns `address` == [NATIVE_ADDRESS].
pub fn is_native(address: &Address) -> bool {
    *address == NATIVE_ADDRESS
}

/// Returns (first_native, last_native).
pub fn is_native_path(path: &[Address]) -> (bool, bool) {
    (is_native(path.first().unwrap()), is_native(path.last().unwrap()))
}

/// Replaces all [NATIVE_ADDRESS] with `weth`.
pub fn map_native(path: &[Address], weth: Address) -> Vec<Address> {
    path.iter().cloned().map(|a| if is_native(&a) { weth } else { a }).collect()
}
