use ethers::prelude::*;
use std::sync::Arc;
use uniswap::{contracts::address, Amount, Dex, Protocol, NATIVE_TOKEN_ADDRESS};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let protocol = Protocol::UniswapV2;
    let client = Arc::new({
        let provider = MAINNET.provider();
        let wallet = "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9"
            .parse::<LocalWallet>()?
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} {:?}", chain, protocol);

    // get contract addresses from address book
    let usdc = address("USDC", chain);

    let mut dex = Dex::new(client.clone(), chain, protocol);

    // swap amount
    let raw_amount = U256::exp10(18);
    let amount = Amount::ExactIn(raw_amount);
    println!("Amount: {:?}", amount);

    // construct swap path
    // specify native ETH by using NATIVE_TOKEN_ADDRESS or Address::repeat_byte(0xee)
    let eth = NATIVE_TOKEN_ADDRESS;
    let path = vec![eth, usdc];
    println!("Path: {:?}", path);

    println!("Creating swap tx");
    let swap_call = dex.swap(amount, 0.5, path, None, None).await?;
    println!("Swap tx: {:#?}", swap_call.tx);

    // deposit and withdraw WETH
    println!("Creating deposit tx");
    let deposit_call = dex.weth_deposit(raw_amount)?;
    println!("Deposit tx: {:#?}", deposit_call.tx);

    println!("Creating withdraw tx");
    let withdraw_call = dex.weth_withdraw(raw_amount)?;
    println!("Deposit tx: {:#?}", withdraw_call.tx);

    Ok(())
}
