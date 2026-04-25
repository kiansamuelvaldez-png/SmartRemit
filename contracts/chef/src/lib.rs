
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address};

#[contract]
pub struct Scan2Cook;

#[contractimpl]
impl Scan2Cook {

    // Store whether a user has paid for a recipe
    pub fn pay_and_unlock(env: Env, user: Address, recipe_id: Symbol) {
        user.require_auth();

        let key = (user.clone(), recipe_id.clone());

        // Mark recipe as unlocked
        env.storage().instance().set(&key, &true);
    }

    // Check if user has access to recipe
    pub fn has_access(env: Env, user: Address, recipe_id: Symbol) -> bool {
        let key = (user, recipe_id);
        env.storage().instance().get(&key).unwrap_or(false)
    }
}

