use ethers::{prelude::*, utils::format_units};
use eyre::ContextCompat;
use std::sync::Arc;
use uniswap_rs::{contracts::addresses::address, Dex, ProtocolType};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let protocol = ProtocolType::UniswapV2;
    let client = Arc::new(MAINNET.provider());

    println!("Using {chain:?} {protocol:?}");

    // get contract addresses from address book
    let weth = address("WETH", chain);
    let usdc = address("USDC", chain);

    println!("Getting ETH/USDC pair info:");
    let dex = Dex::new_with_chain(client, chain, protocol).unwrap();
    let mut pair = dex.pair_for(weth, usdc);

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
