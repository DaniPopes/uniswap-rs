use ethers::prelude::*;
use std::sync::Arc;
use uniswap_rs::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Goerli;
    let client = Arc::new({
        let provider = GOERLI.provider();
        // FIXME: Replace with own private key / wallet.
        let wallet = "1aeda1fc24f9ea6809619040f1d3374255e17a0a3d9c75d85e0ba676ea42ccbd"
            .parse::<LocalWallet>()?
            .with_chain_id(chain);
        println!("Wallet: {:?}", wallet.address());

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {chain:?}");

    // get contract addresses from address book
    let usdc = address("USDC", chain);

    // instantiate a new router
    let mut router = {
        let address = Address::repeat_byte(0x11);
        UniversalRouter::new(client, address)
    };

    // construct the call
    let recipient = Address::repeat_byte(0x33);
    let value = U256::from(5) * U256::exp10(16); // 0.05 ETH
    let deadline = Some(300);
    let swap_call = router
        .v2_swap_exact_in(false, recipient, value, U256::zero(), vec![NATIVE_ADDRESS, usdc], true)
        // .command_name(allow_revert, ...params) ...
        .call(deadline)
        .value(value);

    // simulate the transaction
    swap_call.call().await?;
    println!("Simulation successful");

    // send the transaction and await inclusion in a block
    println!("Sending transaction...");
    let pending_tx = swap_call.send().await?;
    println!("Transaction sent successfully, awaiting inclusion...");
    let receipt = pending_tx.await?.expect("swap transaction was dropped from mempool");
    println!("Swap successful. Receipt: {receipt:#?}");

    Ok(())
}
