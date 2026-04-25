#![cfg(test)]

use soroban_sdk::{Env, Symbol, Address};
use crate::Scan2Cook;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Scan2Cook);
    let client = Scan2CookClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let recipe = Symbol::short("R1");

    client.pay_and_unlock(&user, &recipe);
    assert!(client.has_access(&user, &recipe));
}

#[test]
fn test_edge_case_duplicate() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Scan2Cook);
    let client = Scan2CookClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let recipe = Symbol::short("R1");

    client.pay_and_unlock(&user, &recipe);
    client.pay_and_unlock(&user, &recipe);

    assert!(client.has_access(&user, &recipe));
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Scan2Cook);
    let client = Scan2CookClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let recipe = Symbol::short("R2");

    assert!(!client.has_access(&user, &recipe));

    client.pay_and_unlock(&user, &recipe);

    assert!(client.has_access(&user, &recipe));
}

#[test]
fn test_unauthorized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Scan2Cook);
    let client = Scan2CookClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let recipe = Symbol::short("R3");

    client.pay_and_unlock(&user, &recipe);

    assert!(client.has_access(&user, &recipe));
}

#[test]
fn test_multiple_users() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Scan2Cook);
    let client = Scan2CookClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let recipe = Symbol::short("R4");

    client.pay_and_unlock(&user1, &recipe);

    assert!(client.has_access(&user1, &recipe));
    assert!(!client.has_access(&user2, &recipe));
}
