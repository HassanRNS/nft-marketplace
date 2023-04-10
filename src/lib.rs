#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
   pub fn ping(env: Env, from: Symbol) -> Vec<Symbol> {
       vec![&env, Symbol::short("Hello"), from]
   }

   pub fn mint_nft(env: Env, name: Symbol, token_uri: Symbol) {
       env.storage().set(&name, &token_uri);
   }

   pub fn get_nft(env:Env, name: Symbol) -> Symbol {
       let symbol_result = env.storage().get_unchecked(&name);
       let symbol: Symbol = match symbol_result {
           Ok(s) => s,
           Err(_e) => {
               Symbol::short("error")
           }
       };
       return symbol
   }
}
