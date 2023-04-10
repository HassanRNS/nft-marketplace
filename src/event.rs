use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn set_admin(e: &Env, admin: Address, new_admin: Address) {
    let topics = (Symbol::short("set_admin"), admin);
    e.events().publish(topics, new_admin);
}
