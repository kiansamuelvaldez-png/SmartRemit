#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct SmartRemitContract;

#[contractimpl]
impl SmartRemitContract {

    // Send money (core MVP)
    pub fn send_remittance(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        // Simple transfer logic (demo purpose)
        let key = (from.clone(), to.clone());
        let mut ledger: Map<(Address, Address), i128> =
            env.storage().instance().get(&Symbol::short("TX")).unwrap_or(Map::new(&env));

        ledger.set(key, amount);
        env.storage().instance().set(&Symbol::short("TX"), &ledger);
    }

    // Check transaction
    pub fn check_transaction(env: Env, from: Address, to: Address) -> i128 {
        let ledger: Map<(Address, Address), i128> =
            env.storage().instance().get(&Symbol::short("TX")).unwrap();

        ledger.get((from, to)).unwrap_or(0)
    }
}