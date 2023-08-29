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
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

// YOUR PRIVATE KEY FILE PATH.
const ENV_PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";
// RPC URL FOR A STYLUS CHAIN ENDPOINT.
const ENV_RPC_URL: &str = "RPC_URL";
// DEPLOYED PROGRAM ADDRESS FOR STYLUS-HELLO-WORLD.
const ENV_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";

// 7f85fb7f42a0c0d40431cc0f7dfdf88be6495e67

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let priv_key_path = std::env::var(ENV_PRIV_KEY_PATH)
        .map_err(|_| eyre!("No {} env var set", ENV_PRIV_KEY_PATH))?;
    let rpc_url =
        std::env::var(ENV_RPC_URL).map_err(|_| eyre!("No {} env var set", ENV_RPC_URL))?;
    let program_address = std::env::var(ENV_PROGRAM_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", ENV_PROGRAM_ADDRESS))?;

    // let privkey = read_secret_from_file(&priv_key_path)?;
    // let wallet = LocalWallet::from_str(&privkey)?;
    // let chain_id = provider.get_chainid().await?.as_u64();
    // let client = Arc::new(SignerMiddleware::new(
    //     provider,
    //     wallet.clone().with_chain_id(chain_id),
    // ));

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;
    abigen!(
        Sha256Hasher,
        r#"[
            function sha256(bytes memory input) external view returns (bytes32)
        ]"#
    );
    let hasher = Sha256Hasher::new(address, provider.into());
    let input = Bytes::from(b"The quick brown fox jumps over the lazy dog");
    let resp: [u8; 32] = hasher.sha_256(input).call().await?;

    println!("Resp: {:?}", Bytes::from(resp));
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
