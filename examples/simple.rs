use ethers::prelude::*;
use std::sync::Arc;
use uniswap::{Amount, Dex, Protocol};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let client = Arc::new({
        let provider = MAINNET.provider();
        let wallet = "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9"
            .parse::<LocalWallet>()?
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });
    let protocol = Protocol::UniswapV2;

    println!("Connected to {:?}", chain);

    let mut dex = Dex::new(client, chain, protocol);

    // swap amount
    let raw_amount = U256::exp10(18);
    let amount = Amount::ExactIn(raw_amount);
    println!("Amount: {:?}", amount);

    // construct swap path
    // specify native ETH by using Address::zero()
    let eth = Address::zero();
    let weth: Address = "C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let usdc: Address = "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse()?;
    let path = vec![eth, usdc];
    println!("Path: {:?}", path);

    println!("Creating swap tx");
    let swap_call = dex.swap(amount, 100.0, path, None, None).await?;
    println!("Swap call: {:#?}", swap_call.tx);

    // also supports depositing and withdrawing WETH
    println!("Creating deposit tx");
    let path = vec![eth, weth];
    let deposit_call_1 = dex.swap(amount, 100.0, path, None, None).await?;
    // or by using the weth methods directly
    let deposit_call_2 = dex.weth_deposit(raw_amount)?;
    assert_eq!(deposit_call_1.tx, deposit_call_2.tx);
    println!("Deposit tx: {:#?}", deposit_call_1.tx);

    Ok(())
}
