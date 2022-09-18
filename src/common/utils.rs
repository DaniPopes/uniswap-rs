//! Utils

use crate::constants::NATIVE_TOKEN_ADDRESS;
use ethers::prelude::*;
use std::time::{Duration, SystemTime};

pub(crate) fn now() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("time went backwards")
}

#[inline]
pub(crate) fn is_native(address: &Address) -> bool {
    address == &NATIVE_TOKEN_ADDRESS
}

pub(crate) fn map_native(path: &mut [Address], weth: Address) {
    for a in path.iter_mut() {
        if is_native(a) {
            *a = weth
        }
    }
}

/// Returns (from_native, to_native).
pub(crate) fn is_native_path(path: &[Address]) -> (bool, bool) {
    (is_native(path.first().unwrap()), is_native(path.last().unwrap()))
}
