use ethers::prelude::*;
use eyre::ContextCompat;
use std::sync::Arc;
use uniswap_rs::{Dex, ProtocolType};

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

    // instantiate a new dex
    let dex = Dex::new_with_chain(client, chain, protocol).unwrap();

    println!("WETH address: {:?}", dex.weth().expect("Mainnet WETH not set (should not happen)"));

    let amount = U256::exp10(18);
    println!("Amount: {:?} ETH", ethers::utils::format_units(amount, 18)?);

    // create deposit call
    let deposit_call = dex.weth_deposit(amount)?;

    // send the transaction and await inclusion in a block
    println!("Sending deposit...");
    let pending_tx = deposit_call.send().await?;
    println!("Transaction sent successfully, awaiting inclusion...");
    let receipt = pending_tx.await?.wrap_err("deposit transaction was dropped from mempool")?;
    println!("Successfully deposited {amount} ETH to WETH. Receipt: {receipt:?}");

    Ok(())
}
