#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{
    abi::{Bytes, FixedBytes},
    prelude::*,
};

#[entrypoint]
#[solidity_storage]
pub struct Sha256Hasher;

use sha2::{Digest, Sha256};

#[external]
impl Sha256Hasher {
    pub fn sha256(&self, input: Bytes) -> Result<FixedBytes<32>, Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let x: [u8; 32] = hasher.finalize().as_slice().try_into().unwrap();
        Ok(x.into())
    }
}
