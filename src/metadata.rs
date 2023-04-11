use soroban_sdk::{contracttype, Env, Address};
use std::string::String;

#[contracttype]
pub struct TokenMetadata {
    pub id: u32,
    pub token_uri: String,
    pub owner: Address
}
pub fn get_nft_counter(e: &Env) -> u32 {
    let counter_result = e.storage().get_unchecked(&"nftCounter");
    let counter: u32 = match counter_result {
        Ok(i) => i,
        Err(_e) => 0,
    };
    return counter;
}

pub fn get_base_uri(env: &Env) -> String {
    let uri_result = env.storage().get_unchecked(&"baseURI");
    let uri = match uri_result {
        Ok(s) => s,
        Err(_e) => panic!("Base URI not found!")
    };
    return uri;
}