//! Utils

use super::constants::NATIVE_ADDRESS;
use ethers_core::types::{Address, U256};
use std::time::{Duration, SystemTime};

/// Returns the [Duration] since the UNIX epoch.
#[inline]
pub fn now() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}

/// Returns `deadline` + [`now()`][now].
#[inline]
pub fn get_deadline(deadline: u64) -> U256 {
    U256::from(deadline + now().as_secs())
}

/// Returns `deadline` + [`now()`][now] or [`U256::MAX`](U256).
#[inline]
pub fn get_deadline_opt(deadline: Option<u64>) -> U256 {
    deadline.map(get_deadline).unwrap_or(U256::MAX)
}

/// Returns `address` == [NATIVE_ADDRESS].
#[inline]
pub fn is_native(address: &Address) -> bool {
    *address == NATIVE_ADDRESS
}

/// Returns `(first_native, last_native)`.
#[inline]
pub fn is_native_path(path: &[Address]) -> (bool, bool) {
    (
        path.first().map(is_native).unwrap_or_default(),
        path.last().map(is_native).unwrap_or_default(),
    )
}

/// Replaces all [NATIVE_ADDRESS] in `path` with `weth`.
pub fn map_native(path: &mut [Address], weth: Address) {
    for a in path.iter_mut() {
        if is_native(a) {
            *a = weth
        }
    }
}
