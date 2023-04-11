use soroban_sdk::{contracttype, Symbol, Address};
#[contracttype]
pub struct TokenMetadata {
    pub id: u32,
    pub tokenURI: Symbol,
    pub owner: Address
}
