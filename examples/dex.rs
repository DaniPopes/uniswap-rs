use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
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
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} {:?}", chain, protocol);

    // get contract addresses from address book
    let usdc = address("USDC", chain);

    let mut dex = Dex::new_with_chain(client.clone(), chain, protocol);

    // swap amount
    let raw_amount = U256::exp10(18);
    let amount = Amount::ExactIn(raw_amount);
    println!("Amount: {:?}", amount);

    // construct swap path
    // specify native ETH by using NATIVE_TOKEN_ADDRESS or Address::repeat_byte(0xee)
    let eth = NATIVE_TOKEN_ADDRESS;
    let path = vec![eth, usdc];
    println!("Path: {:?}", path);

    // swap with router
    let swap_call = dex.swap(amount, 0.5, path, None, None).await?;
    log_tx("Swap", swap_call.tx);

    // deposit and withdraw WETH
    let deposit_call = dex.weth_deposit(raw_amount)?;
    log_tx("Deposit", deposit_call.tx);

    let withdraw_call = dex.weth_withdraw(raw_amount)?;
    log_tx("Withdrawal", withdraw_call.tx);

    Ok(())
}

fn log_tx(header: &str, tx: TypedTransaction) {
    println!("{:-^60}", header);
    println!("From: {:?}", tx.from().unwrap());
    println!("To:   {:?}", tx.to().unwrap());
    println!("Data: {:?}", tx.data().unwrap());
}
