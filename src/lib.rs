#![no_std]
mod admin;
mod event;
mod storage_types;

use crate::admin::{has_administrator, read_administrator, write_administrator};

use soroban_sdk::{contractimpl, vec, Address, Env, Symbol, Vec};
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }

    pub fn ping(env: Env, from: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), from]
    }

    pub fn mint_nft(env: Env, name: Symbol, token_uri: Symbol) {
        let admin = read_administrator(&env);
        admin.require_auth();
        env.storage().set(&name, &token_uri);
    }

    pub fn get_nft(env: Env, name: Symbol) -> Symbol {
        let symbol_result = env.storage().get_unchecked(&name);
        let symbol: Symbol = match symbol_result {
            Ok(s) => s,
            Err(_e) => Symbol::short("error"),
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
