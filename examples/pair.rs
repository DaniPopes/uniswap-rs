use ethers::{prelude::*, utils::format_units};
use eyre::ContextCompat;
use std::sync::Arc;
use uniswap_rs::{
    contracts::address,
    v2::{Factory, Pair},
    Dex, ProtocolType,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let protocol = ProtocolType::UniswapV2;
    let client = Arc::new({
        let provider = MAINNET.provider();
        let wallet = "1aeda1fc24f9ea6809619040f1d3374255e17a0a3d9c75d85e0ba676ea42ccbd"
            .parse::<LocalWallet>()?
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} {:?}", chain, protocol);

    // get contract addresses from address book
    let weth = address("WETH", chain);
    let usdc = address("USDC", chain);

    println!("Getting ETH/USDC pair info:");
    let factory = Factory::new_with_chain(chain, protocol).wrap_err("chain not supported")?;
    let _pair = Pair::new_with_factory(client.clone(), factory, weth, usdc)?;
    // or
    let dex = Dex::new_with_chain(client, chain, protocol);
    let mut pair = dex.pair_for(weth, usdc)?;

    pair.sync(true, true).await?;

    let _address = pair.address();
    let _tokens = pair.tokens().wrap_err("could not sync tokens")?;
    let reserves = pair.reserves().wrap_err("could not sync reserves")?;

    // usdc is token0 with 6 decimals, eth is token1 with 18 decimals
    let usdc_reserve: f64 = format_units(U256::from(reserves.0), 6)?.parse()?;
    let eth_reserve: f64 = format_units(U256::from(reserves.1), 18)?.parse()?;

    let price = usdc_reserve / eth_reserve;

    println!("{}", pair);
    println!("Price: ${:.2}", price);

    Ok(())
}
