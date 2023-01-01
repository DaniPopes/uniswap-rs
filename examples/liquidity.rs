use ethers::{
    abi::Detokenize,
    prelude::{builders::ContractCall, *},
};
use eyre::ContextCompat;
use std::sync::Arc;
use uniswap_rs::{constants::NATIVE_ADDRESS, contracts::addresses::address, Dex, ProtocolType};

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
    let weth = address("WETH", chain);
    let usdc = address("USDC", chain);

    // instantiate a new dex
    let dex = Dex::new_with_chain(client.clone(), chain, protocol).unwrap();

    let pair = dex.pair_for(weth, usdc);

    // liquidity amount
    let liquidity = U256::exp10(9);

    println!("Checking approval...");
    let router_address = dex.router().address();
    let allowance = pair.contract().allowance(client.address(), router_address).call().await?;
    if allowance < liquidity {
        // approve the pair
        println!("Approving for removal...");
        let call = pair.contract().approve(dex.router().address(), liquidity);
        let receipt = send(call).await?;
        println!("Contract approved successfully! Receipt: {receipt:#?}");
    } else {
        println!("Already approved, skipping approval");
    }

    // create the remove liquidity transaction
    println!("Removing {liquidity} liquidity...");
    let call = dex
        .remove_liquidity(NATIVE_ADDRESS, usdc, liquidity, 0.into(), 0.into(), None, None)
        .await?;
    println!("Sending swap...");
    let receipt = send(call).await?;
    println!("Successfully removed {liquidity} liquidity from ETH/USDC. Receipt: {receipt:#?}");

    Ok(())
}

async fn send<M: Middleware + 'static, D: Detokenize>(
    call: ContractCall<M, D>,
) -> eyre::Result<TransactionReceipt> {
    let pending_tx = call.send().await?;
    println!("Transaction sent successfully, awaiting inclusion...");
    pending_tx.await?.wrap_err("swap transaction was dropped from mempool")
}
