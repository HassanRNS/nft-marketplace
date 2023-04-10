use soroban_sdk::{contracttype, Symbol};
#[contracttype]
pub struct TokenMetadata {
    pub id: u32,
    pub name: Symbol,
}
