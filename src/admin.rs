use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().set(&key, id);
}
pub fn get_nft_counter(e: &Env) -> u32 {
    let counter_result = e.storage().get_unchecked(&"nftCounter");
    let counter: u32 = match counter_result {
        Ok(i) => i,
        Err(_e) => 0,
    };
    return counter;
}
