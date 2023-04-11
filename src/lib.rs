#![no_std]
mod admin;
mod authorize;
mod event;
mod metadata;
mod storage_types;
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::authorize::{is_authorized, write_authorization};
use crate::metadata::{get_nft_counter, total_supply, TokenMetadata};

use soroban_sdk::{contractimpl, vec, Address, Env, Symbol, Vec};
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, admin: Address, supply: u32) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &admin);
        write_authorization(&env, admin, true);
        env.storage().set(&"nftCounter", &(0));
        env.storage().set(&"nftSupply", &supply);
    }

    pub fn ping(env: Env, from: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), from]
    }
    pub fn mint_nft(env: Env, to: Address) {
        if !is_authorized(&env, to.clone()) {
            panic!("Address is not whitelisted");
        }
        to.require_auth();
        let new_token_id = get_nft_counter(&env);

        let metadata = TokenMetadata {
            id: new_token_id,
            name: Symbol::short(""),
        };
        env.storage().set(&metadata.id, &metadata);
        env.storage().set(&"nftCounter", &(new_token_id + 1));
        event::mint(&env, to, metadata.id)
    }

    pub fn update_nft_metadata(env: Env, id: u32, name: Symbol) {
        let admin = read_administrator(&env);
        admin.require_auth();
        let symbol_result = env.storage().get_unchecked(&id);
        let old_metadata: TokenMetadata = match symbol_result {
            Ok(s) => s,
            Err(_e) => TokenMetadata {
                id: (0),
                name: (Symbol::short("ERROR")),
            },
        };
        let new_metadata = TokenMetadata {
            id: old_metadata.id,
            name: name,
        };
        env.storage().set(&new_metadata.id, &new_metadata);
    }

    pub fn get_nft(env: Env, id: u32) -> TokenMetadata {
        let symbol_result = env.storage().get_unchecked(&id);
        let symbol: TokenMetadata = match symbol_result {
            Ok(s) => s,
            Err(_e) => TokenMetadata {
                id: (0),
                name: (Symbol::short("ERROR")),
            },
        };
        return symbol;
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();
        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    pub fn current_nft_supply(env: Env) -> u32 {
        let supply = get_nft_counter(&env);
        return supply;
    }

    pub fn total_nft_supply(env: Env) -> u32 {
        let max_supply = total_supply(&env);
        return max_supply;
    }

    pub fn set_authorization(env: Env, id: Address, authorize: bool) {
        let admin = read_administrator(&env);
        admin.require_auth();
        write_authorization(&env, id.clone(), authorize);
        event::set_authorized(&env, admin, id, authorize);
    }
}
