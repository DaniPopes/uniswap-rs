use ethers::prelude::*;
use std::sync::Arc;
use uniswap_rs::{
    constants::NATIVE_ADDRESS, contracts::addresses::address, Amount, Dex, ProtocolType,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Goerli;
    let protocol = ProtocolType::UniswapV2;
    let client = Arc::new({
        let provider = GOERLI.provider();
        // FIXME: Replace with own private key / wallet.
        let wallet = "1aeda1fc24f9ea6809619040f1d3374255e17a0a3d9c75d85e0ba676ea42ccbd"
            .parse::<LocalWallet>()?
            .with_chain_id(chain);
        println!("Wallet: {:?}", wallet.address());

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {chain:?} {protocol:?}");

    // get contract addresses from address book
    let usdc = address("USDC", chain);

    // instantiate a new dex
    let mut dex = Dex::new_with_chain(client, chain, protocol).unwrap();

    // swap amount
    let raw_amount = U256::exp10(3);
    let amount = Amount::ExactIn(raw_amount);
    println!("Amount: {:?} ETH", ethers::utils::format_units(raw_amount, 3)?);

    // construct swap path
    // specify native ETH by using NATIVE_ADDRESS or Address::repeat_byte(0xee)
    let eth = NATIVE_ADDRESS;
    let path = [eth, usdc];
    println!("Path:   {path:?}");

    // create the swap transaction
    let swap_call = dex.swap(amount, 0.5, &path, None, None).await?;

    // simulate the transaction
    let res = swap_call.call().await?;
    debug_assert_eq!(res[0], raw_amount);
    println!("Simulation successful, will receive {:?} USDC units", res[1]);

    // send the transaction and await inclusion in a block
    println!("Sending transaction...");
    let pending_tx = swap_call.send().await?;
    println!("Transaction sent successfully, awaiting inclusion...");
    let receipt = pending_tx.await?.expect("swap transaction was dropped from mempool");
    println!("Swap successful. Receipt: {:#?}", receipt);

    Ok(())
}
