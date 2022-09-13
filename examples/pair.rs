use ethers::{prelude::*, utils::format_units};
use std::sync::Arc;
use uniswap::{
    contracts::address,
    v2::{Factory, Pair},
    ProtocolType,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let protocol = ProtocolType::UniswapV2;
    let client = Arc::new({
        let provider = MAINNET.provider();
        let wallet = "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9"
            .parse::<LocalWallet>()?
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} {:?}", chain, protocol);

    // get contract addresses from address book
    let weth = address("WETH", chain);
    let usdc = address("USDC", chain);

    println!("Getting ETH/USDC pair info:");
    let factory = Factory::new_with_chain(chain, protocol).unwrap();
    let mut pair = Pair::new_with_factory(client, factory, weth, usdc)?;

    pair.sync(true, true).await?;

    let address = pair.address();
    let tokens = pair.tokens().unwrap();

    // usdc is token0 with 6 decimals, eth is token1 with 18 decimals
    let reserves = pair.reserves().expect("could not sync reserves");
    let usdc_reserve: f64 = format_units(U256::from(reserves.0), 6)?.parse()?;
    let eth_reserve: f64 = format_units(U256::from(reserves.1), 18)?.parse()?;

    let price = usdc_reserve / eth_reserve;

    println!("Address:  {:?}", address);
    println!("USDC:     {:?}", tokens.0);
    println!("ETH:      {:?}", tokens.1);
    println!("Reserves: {:.2} USDC, {:.2} ETH", usdc_reserve, eth_reserve);
    println!("Price:    ${:.2}", price);

    Ok(())
}
