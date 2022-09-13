use std::sync::Arc;

use super::factory::Factory;
use crate::{bindings::i_uniswap_v2_pair::IUniswapV2Pair, errors::LibraryError};
use ethers::prelude::*;

/// Type alias for Result<T, UniswapV2LibraryError>.
type Result<T> = std::result::Result<T, LibraryError>;

/// The UniswapV2 library refactored from the official [@Uniswap/v2-periphery].
///
/// [@Uniswap/v2-periphery]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol
pub struct Library;

impl Library {
    /// Returns sorted token addresses, used to handle return values from pairs sorted in this
    /// order.
    pub fn sort_tokens(a: Address, b: Address) -> Result<(Address, Address)> {
        if a == b {
            return Err(LibraryError::IdenticalAddresses)
        }

        let (a, b) = if a < b { (a, b) } else { (b, a) };

        if a.is_zero() {
            return Err(LibraryError::ZeroAddress)
        }

        Ok((a, b))
    }

    /// Calculates the CREATE2 address for a pair without making any external calls.
    pub fn pair_for(factory: Factory, a: Address, b: Address) -> Result<Address> {
        let (a, b) = Self::sort_tokens(a, b)?;
        Ok(ethers::utils::get_create2_address_from_hash(
            factory.address(),
            ethers::utils::keccak256([a.0, b.0].concat()), // keccak256(abi.encodePacked(a, b))
            factory.pair_codehash().0,
        ))
    }

    /// Fetches and sorts the reserves for a pair.
    pub async fn get_reserves<M: Middleware>(
        client: Arc<M>,
        factory: Factory,
        a: Address,
        b: Address,
    ) -> Result<(U256, U256)> {
        let (address_0, _) = Self::sort_tokens(a, b)?;
        let pair = IUniswapV2Pair::new(Self::pair_for(factory.clone(), a, b)?, client);
        let (reserve_a, reserve_b, _) = pair
            .get_reserves()
            .call()
            .await
            .map_err(|e| LibraryError::ContractError(e.to_string()))?;
        let (reserve_a, reserve_b) = (reserve_a.into(), reserve_b.into());
        if a == address_0 {
            Ok((reserve_a, reserve_b))
        } else {
            Ok((reserve_b, reserve_a))
        }
    }

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other
    /// asset.
    pub fn quote(amount_a: U256, reserve_a: U256, reserve_b: U256) -> Result<U256> {
        if amount_a.is_zero() {
            Err(LibraryError::InsufficientAmount)
        } else if reserve_a.is_zero() || reserve_b.is_zero() {
            Err(LibraryError::InsufficientLiquidity)
        } else {
            Ok((amount_a * reserve_b) / reserve_a)
        }
    }

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of
    /// the other asset.
    pub fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256) -> Result<U256> {
        if amount_in.is_zero() {
            return Err(LibraryError::InsufficientInputAmount)
        }
        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err(LibraryError::InsufficientLiquidity)
        }
        let amount_in_with_fee: U256 = amount_in * 997;
        let numerator: U256 = amount_in_with_fee * reserve_out;
        let denominator: U256 = (reserve_in * 1000) + amount_in_with_fee;
        Ok(numerator / denominator)
    }

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the
    /// other asset.
    pub fn get_amount_in(amount_out: U256, reserve_in: U256, reserve_out: U256) -> Result<U256> {
        if amount_out.is_zero() {
            return Err(LibraryError::InsufficientOutputAmount)
        }
        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err(LibraryError::InsufficientLiquidity)
        }
        let numerator: U256 = (reserve_in * amount_out) * 1000;
        let denominator: U256 = (reserve_out - amount_out) * 997;
        Ok((numerator / denominator) + 1)
    }

    /// Performs chained get_amount_out calculations on any number of pairs.
    pub async fn get_amounts_out<M: Middleware>(
        client: Arc<M>,
        factory: Factory,
        amount_in: U256,
        path: Vec<Address>,
    ) -> Result<Vec<U256>> {
        let l = path.len();
        if l < 2 {
            return Err(LibraryError::InvalidPath)
        }
        let mut amounts = vec![U256::zero(); l];
        amounts[0] = amount_in;
        for i in 0..l - 1 {
            let (reserve_in, reserve_out) =
                Self::get_reserves(client.clone(), factory, path[i], path[i + 1]).await?;
            amounts[i + 1] = Self::get_amount_out(amounts[i], reserve_in, reserve_out)?;
        }
        Ok(amounts)
    }

    /// Performs chained get_amount_in calculations on any number of pairs.
    pub async fn get_amounts_in<M: Middleware>(
        client: Arc<M>,
        factory: Factory,
        amount_out: U256,
        path: Vec<Address>,
    ) -> Result<Vec<U256>> {
        let l = path.len();
        if l < 2 {
            return Err(LibraryError::InvalidPath)
        }
        let mut amounts = vec![U256::zero(); l];
        amounts[l - 1] = amount_out;
        for i in (1..l).rev() {
            let (reserve_in, reserve_out) =
                Self::get_reserves(client.clone(), factory, path[i - 1], path[i]).await?;
            amounts[i - 1] = Self::get_amount_in(amounts[i], reserve_in, reserve_out)?;
        }
        Ok(amounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolType;
    use std::sync::Arc;

    static FACTORY: Lazy<Factory> =
        Lazy::new(|| Factory::new_with_chain(Chain::Mainnet, ProtocolType::UniswapV2).unwrap());
    static WETH: Lazy<Address> =
        Lazy::new(|| "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap());
    static USDC: Lazy<Address> =
        Lazy::new(|| "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse().unwrap());
    static WETH_USDC: Lazy<Address> =
        Lazy::new(|| "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse().unwrap());

    #[test]
    fn can_sort_tokens() {
        let addr = Address::repeat_byte(0x69);
        let (a, b) = (addr, addr);
        let res = Library::sort_tokens(a, b);
        assert!(matches!(res.unwrap_err(), LibraryError::IdenticalAddresses));

        let res = Library::sort_tokens(Address::zero(), b);
        assert!(matches!(res.unwrap_err(), LibraryError::ZeroAddress));

        let res = Library::sort_tokens(a, Address::zero());
        assert!(matches!(res.unwrap_err(), LibraryError::ZeroAddress));

        let (a, b) = (Address::random(), Address::random());
        Library::sort_tokens(a, b).unwrap();
    }

    #[test]
    fn can_get_pair_for() {
        // let factory: Address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse().unwrap();
        // let weth: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap();
        // let usdc: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse().unwrap();
        // let weth_usdc: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse().unwrap();

        assert_eq!(Library::pair_for(*FACTORY, *WETH, *USDC).unwrap(), *WETH_USDC);
    }

    async fn get_weth_usdc_reserves<M: Middleware>(client: Arc<M>) -> (U256, U256) {
        Library::get_reserves(client, *FACTORY, *WETH, *USDC).await.unwrap()
    }

    #[tokio::test]
    async fn can_get_reserves() {
        // let anvil = Anvil::new().block_time(2u64).port(8544u16).spawn();
        // let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        let provider = Arc::new(MAINNET.provider());
        get_weth_usdc_reserves(provider).await;
    }

    #[test]
    fn can_quote() {
        let base = U256::exp10(18);
        let amount_a = U256::from(100) * base;
        let reserve_a = U256::from(1000) * base;
        let reserve_b = U256::from(5000) * base;

        let res = Library::quote(U256::zero(), reserve_a, reserve_b);
        assert!(matches!(res.unwrap_err(), LibraryError::InsufficientAmount));

        let res = Library::quote(amount_a, U256::zero(), reserve_b);
        assert!(matches!(res.unwrap_err(), LibraryError::InsufficientLiquidity));

        let res = Library::quote(amount_a, U256::zero(), U256::zero());
        assert!(matches!(res.unwrap_err(), LibraryError::InsufficientLiquidity));

        let res = Library::quote(amount_a, reserve_a, U256::zero());
        assert!(matches!(res.unwrap_err(), LibraryError::InsufficientLiquidity));

        let amount_b = Library::quote(amount_a, reserve_a, reserve_b).unwrap();

        assert_eq!(amount_b, (amount_a * reserve_b) / reserve_a);
    }

    #[tokio::test]
    async fn can_quote_async() {
        let provider = Arc::new(MAINNET.provider());

        let weth_amount = U256::exp10(18);
        let (weth_reserve, usdc_reserve) = get_weth_usdc_reserves(provider.clone()).await;
        Library::quote(weth_amount, weth_reserve, usdc_reserve).unwrap();
    }

    #[tokio::test]
    async fn can_get_amount() {
        let provider = Arc::new(MAINNET.provider());

        let weth_amount = U256::exp10(18);
        let (weth_reserve, usdc_reserve) = get_weth_usdc_reserves(provider.clone()).await;
        let usdc_amount = Library::get_amount_out(weth_amount, weth_reserve, usdc_reserve).unwrap();
        Library::get_amount_in(usdc_amount, usdc_reserve, weth_reserve).unwrap();
    }

    #[tokio::test]
    async fn can_get_amounts() {
        let client = Arc::new(MAINNET.provider());

        let weth_amount = U256::exp10(18);
        let path_1 = vec![*WETH, *USDC];
        let path_2 = vec![*USDC, *WETH];
        let usdc_amount =
            Library::get_amounts_out(client.clone(), *FACTORY, weth_amount, path_1).await.unwrap();
        Library::get_amounts_in(client, *FACTORY, usdc_amount[0], path_2).await.unwrap();
    }
}
