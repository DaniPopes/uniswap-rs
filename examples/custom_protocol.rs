use ethers::prelude::*;
use std::sync::Arc;
use uniswap_rs::{Dex, ProtocolType};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let chain = Chain::Goerli;
    let client = Arc::new({
        let provider = GOERLI.provider();
        // FIXME: Replace with own private key / wallet.
        let wallet = "1aeda1fc24f9ea6809619040f1d3374255e17a0a3d9c75d85e0ba676ea42ccbd"
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
    // FIXME: The hash of the deployment code of the pairs
    let pair_code_hash: H256 =
        "0x7777777777777777777777777777777777777777777777777777777777777777".parse()?;

    let my_protocol = ProtocolType::new(my_factory, my_router, is_v2, pair_code_hash);

    let dex = Dex::new_with_chain(client, chain, my_protocol);

    println!("Using dex: {:#?}", dex);

    Ok(())
}
