//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes},
};
use eyre::eyre;
use futures::future::join_all;
use std::str::FromStr;
use std::sync::Arc;
use std::{
    io::{BufRead, BufReader},
    time::Instant,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let program_address = "67de7fa8b521d17ce03dbfaa4964335b3a8ddf13".to_string();
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;
    abigen!(
        Sha256Hasher,
        r#"[
            function sha256(bytes calldata input) external view returns (bytes32)
        ]"#
    );
    let hasher = Sha256Hasher::new(address, provider.into());

    let start = Instant::now();
    let input = Bytes::from(b"The quick brown fox jumps over the lazy dog");
    let resp: [u8; 32] = hasher.sha_256(input).call().await.unwrap();
    let end = Instant::now();
    println!(
        "Resp: {:?}, took: {:?}",
        Bytes::from(resp),
        end.duration_since(start)
    );
    Ok(())
}
