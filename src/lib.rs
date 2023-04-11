#![no_std]
mod admin;
mod event;
mod metadata;
mod storage_types;

use core::ptr::null;

use crate::admin::{get_nft_counter, has_administrator, read_administrator, write_administrator};
use crate::metadata::TokenMetadata;

use soroban_sdk::{contractimpl, vec, Address, Env, Symbol, Vec};
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, admin: Address) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &admin);
        env.storage().set(&"nftCounter", &(0));
    }

    pub fn ping(env: Env, from: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), from]
    }
    pub fn mint_nft(env: Env, to: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();
        let new_token_id = get_nft_counter(&env);

        let metadata = TokenMetadata {
            id: new_token_id,
            tokenURI: Symbol::short(""),
            owner: to,
        };
        env.storage().set(&metadata.id, &metadata);
        env.storage().set(&"nftCounter", &(new_token_id + 1));
    }

    pub fn get_nft(env: Env, id: u32) -> TokenMetadata {
        let symbol_result = env.storage().get_unchecked(&id);
        let symbol: TokenMetadata = match symbol_result {
            Ok(s) => s,
            Err(_e) => TokenMetadata {
                id: (0),
                tokenURI: (Symbol::short("ERROR")),
                owner: ,
            },
        };
        return symbol;
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_administrator(&e, &new_admin);
        event::set_admin(&e, admin, new_admin);
    }
}
