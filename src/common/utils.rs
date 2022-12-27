//! Utils

use super::constants::{DEFAULT_DEADLINE_SECONDS, NATIVE_ADDRESS};
use ethers::types::{Address, U256};
use std::time::{Duration, SystemTime};

/// Returns the [Duration] since the UNIX epoch.
pub fn now() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}

/// Returns [`now()`][now] + `deadline` or [`U256::MAX`](U256).
pub fn get_deadline(deadline: Option<u64>) -> U256 {
    deadline.map(|dl| now().as_secs() + dl).unwrap_or(U256::MAX)
}

/// Returns `address` == [NATIVE_ADDRESS].
pub fn is_native(address: &Address) -> bool {
    *address == NATIVE_ADDRESS
}

/// Returns `(first_native, last_native)`.
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
