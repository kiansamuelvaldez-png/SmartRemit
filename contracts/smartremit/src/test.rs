#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_send_remittance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SmartRemitContract);
    let client = SmartRemitContractClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to = Address::random(&env);

    client.send_remittance(&from, &to, &1000);

    let result = client.check_transaction(&from, &to);
    assert_eq!(result, 1000);
}

#[test]
fn test_zero_transaction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SmartRemitContract);
    let client = SmartRemitContractClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to = Address::random(&env);

    let result = client.check_transaction(&from, &to);
    assert_eq!(result, 0);
}

#[test]
fn test_state() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SmartRemitContract);
    let client = SmartRemitContractClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to = Address::random(&env);

    client.send_remittance(&from, &to, &500);

    let result = client.check_transaction(&from, &to);
    assert!(result == 500);
}
