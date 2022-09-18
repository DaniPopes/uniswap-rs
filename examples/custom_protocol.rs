use ethers::prelude::*;
use std::sync::Arc;
use uniswap_rs::{Dex, ProtocolType};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Mainnet;
    let client = Arc::new({
        let provider = MAINNET.provider();
        // FIXME: Replace with own private key / wallet.
        let wallet = "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9"
            .parse::<LocalWallet>()?
            .with_chain_id(chain as u64);

        SignerMiddleware::new(provider, wallet)
    });

    println!("Using {:?} Custom protocol", chain);

    // FIXME: Protocol's factory address
    let my_factory: Address = "0x1234123412341234123412341234123412341234".parse()?;
    // FIXME: Protocol's router address
    let my_router: Address = "0x4321432143214321432143214321432143214321".parse()?;
    // FIXME: Whether the protocol is v2 or v3
    let is_v2 = true;

    let my_protocol = ProtocolType::new(my_factory, my_router, is_v2, None);

    let mut dex = Dex::new_with_chain(client, chain, my_protocol);

    dex.set_weth().await?;
    // or
    let my_weth: Address = "0x8888888888888888888888888888888888888888".parse()?;
    dex.set_weth_sync(my_weth);

    println!("Set WETH to: {:?}", dex.weth());

    Ok(())
}
