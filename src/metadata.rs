use soroban_sdk::{contracttype, Env, Symbol};
#[contracttype]
pub struct TokenMetadata {
    pub id: u32,
    pub name: Symbol,
}
pub fn get_nft_counter(e: &Env) -> u32 {
    let counter_result = e.storage().get_unchecked(&"nftCounter");
    let counter: u32 = match counter_result {
        Ok(i) => i,
        Err(_e) => 0,
    };
    return counter;
}
