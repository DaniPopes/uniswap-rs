use super::factory::Factory;
use crate::{
    bindings::i_uniswap_v2_pair::IUniswapV2Pair,
    errors::{Error, Result},
};
use ethers::{core::abi::Tokenizable, prelude::*};
use std::cmp::Ordering;

/// The UniswapV2 library refactored from the official [@Uniswap/v2-periphery].
///
/// [@Uniswap/v2-periphery]: https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol
pub struct Library;

impl Library {
    /// Returns sorted token addresses, used to handle return values from pairs sorted in this
    /// order.
    #[inline]
    pub fn sort_tokens(a: Address, b: Address) -> (Address, Address) {
        match a.cmp(&b) {
            Ordering::Less => (a, b),
            _ => (b, a),
        }
    }

    /// Calculates the CREATE2 address for a pair without making any external calls.
    pub fn pair_for<M: Middleware>(
        factory: &Factory<M>,
        mut a: Address,
        mut b: Address,
    ) -> Address {
        (a, b) = Self::sort_tokens(a, b);

        let from = factory.address();
        // keccak256(abi.encodePacked(a, b))
        let salt = ethers::utils::keccak256([a.0, b.0].concat());
        let init_code_hash = factory.pair_code_hash(None).0;

        ethers::utils::get_create2_address_from_hash(from, salt, init_code_hash)
    }

    /// Fetches and sorts the reserves for a pair.
    pub async fn get_reserves<M: Middleware>(
        factory: &Factory<M>,
        a: Address,
        b: Address,
    ) -> Result<(U256, U256)> {
        let (address_0, _) = Self::sort_tokens(a, b);
        let pair = IUniswapV2Pair::new(Self::pair_for(factory, a, b), factory.client());
        let r = pair.get_reserves().call().await?;
        let (reserve_a, reserve_b) = (r.0.into(), r.1.into());
        if a == address_0 {
            Ok((reserve_a, reserve_b))
        } else {
            Ok((reserve_b, reserve_a))
        }
    }

    /// Fetches and sorts the reserves for multiple pairs. Makes only 1 call to the client by using
    /// [Multicall].
    pub async fn get_reserves_multi<M: Middleware>(
        factory: &Factory<M>,
        path: &[Address],
    ) -> Result<Vec<(U256, U256)>> {
        let len = match path.len() {
            0 | 1 => return Err(Error::InvalidPath),
            2 => {
                // avoid multicall for only 1 call
                let (a, b) = (path[0], path[1]);
                let res = Self::get_reserves(factory, a, b).await?;
                return Ok(vec![res])
            }
            l => l - 1,
        };

        let client = factory.client();
        let mut multicall =
            Multicall::new(client.clone(), None).await?.version(MulticallVersion::Multicall);
        // whether to sort the reserves later
        let mut sorted = Vec::with_capacity(len);

        let pair = IUniswapV2Pair::new(Address::zero(), client);
        let call = pair.get_reserves();
        for slice in path.windows(2) {
            let (a, b) = (slice[0], slice[1]);

            let (address_0, _) = Self::sort_tokens(a, b);
            sorted.push(address_0 == b);

            let mut call = call.clone();
            call.tx.set_to(Self::pair_for(factory, a, b));
            multicall.add_call(call, false);
        }

        multicall
            .call_raw()
            .await?
            .into_iter()
            .zip(sorted)
            .map(|(token, sort)| {
                let (a, b): (U256, U256) = Tokenizable::from_token(token)?;
                Ok(if sort { (b, a) } else { (a, b) })
            })
            .collect()
    }

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other
    /// asset.
    pub fn quote(amount_a: U256, reserve_a: U256, reserve_b: U256) -> Result<U256> {
        if reserve_a.is_zero() || reserve_b.is_zero() {
            return Err(Error::InsufficientLiquidity)
        }
        Ok((amount_a * reserve_b) / reserve_a)
    }

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of
    /// the other asset.
    pub fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256) -> Result<U256> {
        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err(Error::InsufficientLiquidity)
        }
        let amount_in_with_fee = amount_in * 997;
        let numerator = amount_in_with_fee * reserve_out;
        let denominator = reserve_in * 1000 + amount_in_with_fee;
        Ok(numerator / denominator)
    }

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the
    /// other asset.
    pub fn get_amount_in(amount_out: U256, reserve_in: U256, reserve_out: U256) -> Result<U256> {
        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err(Error::InsufficientLiquidity)
        }
        let numerator = reserve_in * amount_out * 1000;
        let denominator = (reserve_out - amount_out) * 997;
        Ok((numerator / denominator) + 1)
    }

    /// Performs chained get_amount_out calculations on any number of pairs.
    pub async fn get_amounts_out<M: Middleware>(
        factory: &Factory<M>,
        amount_in: U256,
        path: &[Address],
    ) -> Result<Vec<U256>> {
        let len = path.len();
        if len < 2 {
            return Err(Error::InvalidPath)
        }

        let reserves = Self::get_reserves_multi(factory, path).await?;
        let mut amounts = Vec::with_capacity(len);
        amounts.push(amount_in);
        for (i, (reserve_in, reserve_out)) in reserves.into_iter().enumerate() {
            amounts.push(Self::get_amount_out(amounts[i], reserve_in, reserve_out)?);
        }
        Ok(amounts)
    }

    /// Performs chained get_amount_in calculations on any number of pairs.
    pub async fn get_amounts_in<M: Middleware>(
        factory: &Factory<M>,
        amount_out: U256,
        path: &[Address],
    ) -> Result<Vec<U256>> {
        let len = path.len();
        if len < 2 {
            return Err(Error::InvalidPath)
        }

        let reserves = Self::get_reserves_multi(factory, path).await?;
        let mut amounts = vec![U256::zero(); len];
        amounts[len - 1] = amount_out;
        for (i, (reserve_in, reserve_out)) in reserves.into_iter().enumerate().rev() {
            amounts[i] = Self::get_amount_in(amounts[i + 1], reserve_in, reserve_out)?;
        }
        Ok(amounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtocolType;

    static FACTORY: Lazy<Factory<Provider<Http>>> = Lazy::new(|| {
        Factory::new_with_chain(MAINNET.provider().into(), Chain::Mainnet, ProtocolType::UniswapV2)
            .unwrap()
    });
    static WETH: Lazy<Address> =
        Lazy::new(|| "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap());
    static USDC: Lazy<Address> =
        Lazy::new(|| "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse().unwrap());
    static WETH_USDC: Lazy<Address> =
        Lazy::new(|| "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse().unwrap());

    #[test]
    fn can_sort_tokens() {
        let (a, b) = (Address::random(), Address::random());
        let (res_a, res_b) = Library::sort_tokens(a, b);
        match a.cmp(&b) {
            Ordering::Less | Ordering::Equal => {
                assert_eq!(res_a, a);
                assert_eq!(res_b, b);
            }
            Ordering::Greater => {
                assert_eq!(res_a, b);
                assert_eq!(res_b, a);
            }
        }
    }

    #[test]
    fn can_get_pair_for() {
        assert_eq!(Library::pair_for(&*FACTORY, *WETH, *USDC), *WETH_USDC);
    }

    async fn get_weth_usdc_reserves() -> (U256, U256) {
        Library::get_reserves(&*FACTORY, *WETH, *USDC).await.unwrap()
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn can_get_reserves() {
        get_weth_usdc_reserves().await;
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn can_get_reserves_multi() {
        let path = [*WETH, *USDC, *WETH, *USDC];
        Library::get_reserves_multi(&*FACTORY, &path).await.unwrap();
    }

    #[test]
    fn can_quote() {
        let base = U256::exp10(18);
        let amount_a = U256::from(100) * base;
        let reserve_a = U256::from(1000) * base;
        let reserve_b = U256::from(5000) * base;

        let res = Library::quote(amount_a, U256::zero(), reserve_b);
        assert!(matches!(res.unwrap_err(), Error::InsufficientLiquidity));

        let res = Library::quote(amount_a, U256::zero(), U256::zero());
        assert!(matches!(res.unwrap_err(), Error::InsufficientLiquidity));

        let res = Library::quote(amount_a, reserve_a, U256::zero());
        assert!(matches!(res.unwrap_err(), Error::InsufficientLiquidity));

        let amount_b = Library::quote(amount_a, reserve_a, reserve_b).unwrap();

        assert_eq!(amount_b, (amount_a * reserve_b) / reserve_a);
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn can_quote_async() {
        let weth_amount = U256::exp10(18);
        let (weth_reserve, usdc_reserve) = get_weth_usdc_reserves().await;
        Library::quote(weth_amount, weth_reserve, usdc_reserve).unwrap();
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn can_get_amount() {
        let weth_amount = U256::exp10(18);
        let (weth_reserve, usdc_reserve) = get_weth_usdc_reserves().await;
        let usdc_amount = Library::get_amount_out(weth_amount, weth_reserve, usdc_reserve).unwrap();
        Library::get_amount_in(usdc_amount, usdc_reserve, weth_reserve).unwrap();
    }

    #[tokio::test]
    #[ignore = "async test"]
    async fn can_get_amounts() {
        let weth_amount = U256::exp10(18);
        let path_1 = [*WETH, *USDC, *WETH, *USDC];
        let path_2 = [*USDC, *WETH, *USDC, *WETH];
        let usdc_amount = Library::get_amounts_out(&*FACTORY, weth_amount, &path_1).await.unwrap();
        Library::get_amounts_in(&*FACTORY, usdc_amount[0], &path_2).await.unwrap();
    }
}
