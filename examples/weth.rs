use ethers::{
    abi::Detokenize,
    prelude::{builders::ContractCall, *},
};
use eyre::ContextCompat;
use std::sync::Arc;
use uniswap::{Dex, ProtocolType};

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

    // instantiate a new dex
    let dex = Dex::new_with_chain(client, chain, protocol);

    let amount = U256::exp10(18);
    println!("Amount: {:?} ETH", ethers::utils::format_units(amount, 18)?);

    // create deposit call
    let deposit_call = dex.weth_deposit(amount)?;

    // create withdrawal call
    let withdraw_call = dex.weth_withdraw(amount)?;

    println!("Created transactions");

    println!("Sending deposit...");
    let receipt = send_tx(deposit_call).await?;
    println!("Successfully deposited {} ETH to WETH. Receipt: {:?}", amount, receipt);

    println!("Sending withdrawal...");
    let receipt = send_tx(withdraw_call).await?;
    println!("Successfully withdrew {} ETH from WETH. Receipt: {:?}", amount, receipt);

    Ok(())
}

async fn send_tx<M: Middleware + 'static, D: Detokenize>(
    call: ContractCall<M, D>,
) -> eyre::Result<TransactionReceipt> {
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.wrap_err("tx was dropped from mempool")?;
    Ok(receipt)
}
