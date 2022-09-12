//! Constants

use ethers::core::types::{Address, U256};

/// [0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE](https://etherscan.io/address/0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE)
///
/// A unique address to differentiate the native token from any other ERC20 token.
pub const NATIVE_TOKEN_ADDRESS: Address = Address::repeat_byte(0xee);

pub(crate) const MIN_DEADLINE_SECONDS: u64 = 30;
pub(crate) const DEFAULT_DEADLINE_SECONDS: u64 = 1800;
pub(crate) const BPS_U256: U256 = U256([10_000u64, 0, 0, 0]);
