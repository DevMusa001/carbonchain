#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String};

#[contract]
pub struct MrvOracle;

#[contractimpl]
impl MrvOracle {
    pub fn update_mrv_data(_env: Env, _oracle: Address, _project_id: String, _tonnes: i128) {
        // TODO: Implementation
    }
}
