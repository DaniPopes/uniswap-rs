use ethers::prelude::*;
use std::sync::Arc;
use uniswap::{constants::NATIVE_TOKEN_ADDRESS, contracts::address, Amount, Dex, ProtocolType};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let protocol = ProtocolType::UniswapV2;
    let client = Arc::new({
        let provider = MAINNET.provider();
        let wallet = "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9"
            .parse::<LocalWallet>()?
            .with_chain_id(chain);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} {:?}", chain, protocol);

    // get contract addresses from address book
    let usdc = address("USDC", chain);

    // instantiate a new dex
    let mut dex = Dex::new_with_chain(client, chain, protocol);

    // swap amount
    let raw_amount = U256::exp10(18);
    let amount = Amount::ExactIn(raw_amount);
    println!("Amount: {:?} ETH", ethers::utils::format_units(raw_amount, 18)?);

    // construct swap path
    // specify native ETH by using NATIVE_TOKEN_ADDRESS or Address::repeat_byte(0xee)
    let eth = NATIVE_TOKEN_ADDRESS;
    let path = vec![eth, usdc];
    println!("Path:   {:?}", path);

    // create the swap transaction
    let swap_call = dex.swap(amount, 0.5, path, None, None).await?;

    // send the transaction and await inclusion in a block
    println!("Sending transaction...");
    let pending_tx = swap_call.send().await?;
    println!("Transaction sent successfully, awaiting inclusion...");
    let receipt = pending_tx.await?.expect("swap transaction was dropped from mempool");
    println!("Successfully swapped {} ETH to USDC. Receipt: {:?}", raw_amount, receipt);

    Ok(())
}
